//! MPK
mod err;
pub use err::Error;

macro_rules! features {
  ($($f:literal $n:ident $o:ident),+) => {
    $(
      #[cfg(feature = $f)]
      pub use $o as $n;
    )*
  };
}

features!("audio" audio mpk_audio,
	  "codec" codec mpk_codec,
	  "config" config mpk_config,
	  "analysis" analysis mpk_analysis,
	  "db" db mpk_db,
	  "vm" vm mpk_vm,
	  "engine" engine mpk_engine,
	  "flate" flate mpk_flate,
	  "hash" hash mpk_hash,
	  "gear" gear mpk_gear,
	  "http" http mpk_http,
	  "jack" jack mpk_jack,
	  "midi" midi mpk_midi,
	  "osc" osc mpk_osc,
	  "parser" parser mpk_parser,
	  "repl" repl mpk_repl,
	  "sesh" sesh mpk_sesh,
	  "util" util mpk_util);
