mod nest_hole_distribution_encounter8archive_generated;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use flatbuffers::{Verifier, VerifierOptions};
use serde::{Deserialize, Serialize};
pub use nest_hole_distribution_encounter8archive_generated::structure;

#[derive(Serialize, Deserialize)]
struct Entry {
    pub EntryIndex: u32,
    pub Species: u32,
    pub AltForm: u32,
    pub Level: u32,
    pub DynamaxLevel: u16,
    pub Field_05: u32,
    pub Field_06: u32,
    pub Field_07: u32,
    pub Field_08: u32,
    pub Field_09: u32,
    pub Field_0A: u32,
    pub Ability: i8,
    pub IsGigantamax: bool,
    pub DropTableID: u64,
    pub BonusTableID: u64,
    pub Probabilities: Vec<u32>,
    pub Gender: i8,
    pub FlawlessIVs: i8,
    pub ShinyFlag: i8,
    pub Field_13: i8,
    pub Field_14: i8,
    pub Nature: i8,
    pub Field_16: u32,
    pub Move0: u32,
    pub Move1: u32,
    pub Move2: u32,
    pub Move3: u32,
    pub DynamaxBoost: f32,
    pub Field_1C: u32,
    pub Field_1D: u32,
    pub Shield: u32,
    pub AdditionalMove1Rate: u32,
    pub AdditionalMove1: u32,
    pub AdditionalMove1PP: u32,
    pub AdditionalMove2Rate: u32,
    pub AdditionalMove2: u32,
    pub AdditionalMove2PP: u32
}

#[derive(Serialize, Deserialize)]
struct Table {
    pub TableID: u64,
    pub GameVersion: u32,
    pub Field_02: i8,
    pub Field_03: i8,
    pub Entries: Vec<Entry>
}

#[derive(Serialize, Deserialize)]
struct Root {
    pub Tables: Vec<Table>
}

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        eprintln!("No IP argument provided");
        std::process::exit(1);
    }

    let ip_str = args.nth(1).unwrap();

    match TcpStream::connect(ip_str) {
        Ok(mut stream) => {
            let msg = "peek 0x2F9EB300 0x23D4\r\n";
            stream.write(msg.as_bytes()).unwrap();
            let mut data = [0 as u8; 0x23D4 * 2];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    let mut buf = hex::decode(&String::from_utf8(Vec::from(data)).unwrap()[..]).unwrap();
                    let nest = structure::root_as_nest_hole_distribution_encounter_8archive(&buf[32..]).expect("Invalid flatbuffer");
                    let root = Root {
                        Tables: nest.Tables().unwrap().iter().map(|t| Table {
                            TableID: t.TableID(),
                            GameVersion: t.GameVersion(),
                            Field_02: t.Field_02(),
                            Field_03: t.Field_03(),
                            Entries: t.Entries().unwrap().iter().map(|e| Entry {
                                EntryIndex: e.EntryIndex(),
                                Species: e.Species(),
                                AltForm: e.AltForm(),
                                Level: e.Level(),
                                DynamaxLevel: e.DynamaxLevel(),
                                Field_05: e.Field_05(),
                                Field_06: e.Field_06(),
                                Field_07: e.Field_07(),
                                Field_08: e.Field_08(),
                                Field_09: e.Field_09(),
                                Field_0A: e.Field_0A(),
                                Ability: e.Ability(),
                                IsGigantamax: e.IsGigantamax(),
                                DropTableID: e.DropTableID(),
                                BonusTableID: e.BonusTableID(),
                                Probabilities: e.Probabilities().unwrap().iter().collect::<Vec<u32>>(),
                                Gender: e.Gender(),
                                FlawlessIVs: e.FlawlessIVs(),
                                ShinyFlag: e.ShinyFlag(),
                                Field_13: e.Field_13(),
                                Field_14: e.Field_14(),
                                Nature: e.Nature(),
                                Field_16: e.Field_16(),
                                Move0: e.Move0(),
                                Move1: e.Move1(),
                                Move2: e.Move2(),
                                Move3: e.Move3(),
                                DynamaxBoost: e.DynamaxBoost(),
                                Field_1C: e.Field_1C(),
                                Field_1D: e.Field_1D(),
                                Shield: e.Shield(),
                                AdditionalMove1Rate: e.AdditionalMove1Rate(),
                                AdditionalMove1: e.AdditionalMove1(),
                                AdditionalMove1PP: e.AdditionalMove1PP(),
                                AdditionalMove2Rate: e.AdditionalMove2Rate(),
                                AdditionalMove2: e.AdditionalMove2(),
                                AdditionalMove2PP: e.AdditionalMove2PP()
                            }).collect::<Vec<Entry>>()
                        }).collect::<Vec<Table>>()
                    };

                    let mut output = File::create("nests_event.json").unwrap();
                    output.write_all(serde_json::to_string_pretty(&root).unwrap().replace("ShinyFlag", "ShinyForced").as_bytes());
                }
                Err(e) => {
                    eprintln!("Failed to receive data: {}", e);
                }
            }

        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}
