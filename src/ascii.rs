use colored::{Colorize, ColoredString};

pub fn neocat(line: i32) -> ColoredString {
    // Neocat design licensed under CC BY-NC-SA 4.0.
    // https://volpeon.ink/projects/emojis/neocat/
    // Thank you Volpeon :neocat_heart:

    /*                                    
                                        
                                        
      .,.                       .,.     
      OKKOxo:.     ...     .:oxOKKO     
      OKKKKKKK0kO0KKKKK0OkOKKKKKKKk     
      OKKKKKKKKKKKKKKKKKKKKKKKKKKKl     
      OKKKKKKKKKKKKKKKKKKKKKKKKKKK'     
      OKKKkl:lOKKKKKKOo:ckKKKKKKK0      
     '0KK0    .KKKKKK,    0KKKKKKK:     
     kKKKKc  .oKKKKKKd.  cKKKKKKKKO     
     KkO0KKK0KKKKKKKKKK0KKKKKOxoOK0     
     KOxloK0:kKk:lOKK0d:OKKkldk0KK0     
     00kdoKKk';.;;.:::x0KKKkox0KKK0     
     cxxOKKKK0Odc:oO0KKKKKKK0kdoOKO     
      ,kKKKKKKKKKKKKKKKKKKKKKKKKKO'     
        ;dOKKKKKKKKKKKKKKKKKKK0kc       
            ';::::::::::::::;'          
                                                      
    */

    if line == 0 {
        return "                                    ".bold().yellow()
    } else if line == 1 {
        return "  .,.                       .,.     ".bold().yellow()
    } else if line == 2 {
        return "  OKKOxo:.     ...     .:oxOKKO     ".bold().yellow()
    } else if line == 3 {
        return "  OKKKKKKK0kO0KKKKK0OkOKKKKKKKk     ".bold().yellow()
    } else if line == 4 {
        return "  OKKKKKKKKKKKKKKKKKKKKKKKKKKKl     ".bold().yellow()
    } else if line == 5 {
        return "  OKKKKKKKKKKKKKKKKKKKKKKKKKKK'     ".bold().yellow()
    } else if line == 6 {
        return "  OKKKkl:lOKKKKKKOo:ckKKKKKKK0      ".bold().yellow()
    } else if line == 7 {
        return " '0KK0    .KKKKKK,    0KKKKKKK:     ".bold().yellow()
    } else if line == 8 {
        return " kKKKKc  .oKKKKKKd.  cKKKKKKKKO     ".bold().yellow()
    } else if line == 9 {
        return " KkO0KKK0KKKKKKKKKK0KKKKKOxoOK0     ".bold().yellow()
    } else if line == 10 {
        return " KOxloK0:kKk:lOKK0d:OKKkldk0KK0     ".bold().yellow()
    } else if line == 11 {
        return format!(" 00kdoKKk'{}x0KKKkox0KKK0     ", ";.;;.:::".red()).bold().yellow();
    } else if line == 12 {
        return format!(" cxxOKKKK0Od{}O0KKKKKKK0kdoOKO     ", "c:o".red()).bold().yellow();
    } else if line == 13 {
        return "  ,kKKKKKKKKKKKKKKKKKKKKKKKKKO'     ".bold().yellow()
    } else if line == 14 {
        return "    ;dOKKKKKKKKKKKKKKKKKKK0kc       ".bold().yellow()
    } else if line == 15 {
        return "        ';::::::::::::::;'          ".bold().yellow()
    } else if line == 16 {
        return "                                    ".bold().yellow()
    }  else {
        "?".white()
    }
}