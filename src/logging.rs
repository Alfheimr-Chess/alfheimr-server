use simplelog::*;
use colored::*;
use crate::args::Arguments;

/// Shows the startup banner
pub fn startup_banner() {
    println!("{}", r#"
     ..                    ..                                     .
  :**888H: `: .xH""  x .d88"     oec :    .uef^"                 @88>
 X   `8888k XX888     5888R     @88888  :d88E                    %8P      ..    .     :       .u    .
'8hx  48888 ?8888     '888R     8"*88%  `888E            .u       .     .888: x888  x888.   .d88B :@8c
'8888 '8888 `8888      888R     8b.      888E .z8k    ud8888.   .@88u  ~`8888~'888X`?888f` ="8888f8888r
 %888>'8888  8888      888R    u888888>  888E~?888L :888'8888. ''888E`   X888  888X '888>    4888>'88"
   "8 '888"  8888      888R     8888R    888E  888E d888 '88%"   888E    X888  888X '888>    4888> '
  .-` X*"    8888      888R     8888P    888E  888E 8888.+"      888E    X888  888X '888>    4888>
    .xhx.    8888      888R     *888>    888E  888E 8888L        888E    X888  888X '888>   .d888L .+
  .H88888h.~`8888.>   .888B .   4888     888E  888E '8888c. .+   888&   "*88%""*88" '888!`  ^"8888*"
 .~  `%88!` '888*~    ^*888%    '888    m888N= 888>  "88888%     R888"    `~    "    `"`       "Y"
       `"     ""        "%       88R     `Y"   888     "YP'       ""
                                 88>          J88"
                                 48           @%
                                 '8         :"
             "#.green());
}

/// Initializes logger and display startup banner
pub fn initialize_logging(options: &Arguments) {
    if !options.no_startup_banner {
        startup_banner();
    }
    TermLogger::init(options.loglevel, Config::default(), TerminalMode::Mixed).unwrap();
}
