use coremidi::{Client, PacketBuffer, VirtualSource};
use crossterm::event::KeyModifiers;
use crossterm::execute;
use crossterm::{
    event::{
        read, Event, KeyCode, KeyEventKind, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
        PushKeyboardEnhancementFlags,
    },
    terminal,
};
use std::collections::{HashMap, HashSet};
use std::io::stdout;

fn midi_packet(is_pressed: bool, channel: u8, note: u8, velocity: u8) -> PacketBuffer {
    let typ = if is_pressed { 0x90 } else { 0x80 };
    let data = &[typ | (channel & 0x0f), note & 0x7f, velocity & 0x7f];

    PacketBuffer::new(0, data)
}

fn keyboard_loop(source: VirtualSource) -> crossterm::Result<()> {
    let key_note: HashMap<KeyCode, u8> = [
        (KeyCode::Char('1'), 47),
        (KeyCode::Char('q'), 48),
        (KeyCode::Char('a'), 49),
        (KeyCode::Char('z'), 50),
        (KeyCode::Char('2'), 50),
        (KeyCode::Char('w'), 51),
        (KeyCode::Char('s'), 52),
        (KeyCode::Char('x'), 53),
        (KeyCode::Char('3'), 53),
        (KeyCode::Char('e'), 54),
        (KeyCode::Char('d'), 55),
        (KeyCode::Char('c'), 56),
        (KeyCode::Char('4'), 56),
        (KeyCode::Char('r'), 57),
        (KeyCode::Char('f'), 58),
        (KeyCode::Char('v'), 59),
        (KeyCode::Char('5'), 59),
        (KeyCode::Char('t'), 60),
        (KeyCode::Char('g'), 61),
        (KeyCode::Char('b'), 62),
        (KeyCode::Char('6'), 62),
        (KeyCode::Char('y'), 63),
        (KeyCode::Char('h'), 64),
        (KeyCode::Char('n'), 65),
        (KeyCode::Char('7'), 65),
        (KeyCode::Char('u'), 66),
        (KeyCode::Char('j'), 67),
        (KeyCode::Char('m'), 68),
        (KeyCode::Char('8'), 68),
        (KeyCode::Char('i'), 69),
        (KeyCode::Char('k'), 70),
        (KeyCode::Char(','), 71),
        (KeyCode::Char('9'), 71),
        (KeyCode::Char('o'), 72),
        (KeyCode::Char('l'), 73),
        (KeyCode::Char('.'), 74),
        (KeyCode::Char('0'), 74),
        (KeyCode::Char('p'), 75),
        (KeyCode::Char(';'), 76),
        (KeyCode::Char('/'), 77),
        (KeyCode::Char('-'), 77),
        (KeyCode::Char('['), 78),
        (KeyCode::Char('\''), 79),
        (KeyCode::Char('='), 80),
        (KeyCode::Char(']'), 81),
        (KeyCode::Char('\\'), 84),
    ]
    .iter()
    .cloned()
    .collect();

    let mut pressed: HashSet<u8> = HashSet::new();

    let note_names = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];

    loop {
        if let Event::Key(event) = read()? {
            if event.code == KeyCode::Esc
                || (event.code == KeyCode::Char('c')
                    && event.modifiers.contains(KeyModifiers::CONTROL))
            {
                break;
            }
            if let Some((is_pressed, note)) =
                key_note.get(&event.code).and_then(|note| match event.kind {
                    KeyEventKind::Press => pressed.insert(*note).then(|| (true, *note)),
                    KeyEventKind::Release => {
                        pressed.remove(&note);
                        // always send release in case it was hanging from before
                        Some((false, *note))
                    }
                    KeyEventKind::Repeat => None,
                })
            {
                println!(
                    "{} {}\r",
                    if is_pressed { "Playing" } else { "Releasing" },
                    note_names[(note % 12) as usize]
                );

                source
                    .received(&midi_packet(is_pressed, 0, note, 127))
                    .unwrap();
            }
        }
    }

    // release all keys
    for note in pressed {
        source.received(&midi_packet(false, 0, note, 127)).unwrap();
    }

    Ok(())
}

fn main() {
    println!("Use your keyboard as an accordion!\nReleasing keys only works in some terminals, like kitty.\nPress ESC to quit.");

    let client = Client::new("Software defined accordion").unwrap();
    let source = client.virtual_source("Software defined accordion").unwrap();

    let mut stdout = stdout();

    execute!(
        stdout,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
    )
    .unwrap();

    terminal::enable_raw_mode().unwrap();
    keyboard_loop(source).unwrap();
    terminal::disable_raw_mode().unwrap();

    execute!(stdout, PopKeyboardEnhancementFlags).unwrap();
}
