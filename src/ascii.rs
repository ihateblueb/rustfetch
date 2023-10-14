use colored::{Colorize, ColoredString};

pub fn get(ascii: String, line: i32) -> ColoredString {
    if ascii == "neocat" {
        return neocat(line);
    } else if ascii == "Ubuntu" {
        return ubuntu(line);
    } else {
        return unknown();
    }
}

pub fn neocat(line: i32) -> ColoredString {
    // Neocat design licensed under CC BY-NC-SA 4.0.
    // https://volpeon.ink/projects/emojis/neocat/
    // Thank you, Volpeon :neocat_heart:

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
        return format!("  cxxOKKKK0Od{}O0KKKKKKK0kdoOKO     ", "c:o".red()).bold().yellow();
    } else if line == 13 {
        return "  ,kKKKKKKKKKKKKKKKKKKKKKKKKKO'     ".bold().yellow()
    } else if line == 14 {
        return "    ;dOKKKKKKKKKKKKKKKKKKK0kc       ".bold().yellow()
    } else if line == 15 {
        return "        ';::::::::::::::;'          ".bold().yellow()
    } else if line == 16 {
        return "                                    ".bold().yellow()
    } else if line == 17 {
        return "                                    ".bold().yellow()
    }  else {
        "?".white()
    }
}

pub fn ubuntu(line: i32) -> ColoredString {
    /*
           ..',,;;;;;;;;,,'..           
       .';;;;;;;;;;;;;;;;;;;;;;'.       
     ';;;;;;;;;;;;;;;;;;xXNNKc;;;;'     
   ';;;;;;;;;;:xOKXXXXKoNMMMMx;;;;;;'   
 .;;;;;;;;;oKNxxMMMWWMMMKKKKx;;;;;;;;;. 
 ;;;;;;;;lXMMMNocc;;;;cd0NMMMXl;;;;;;;; 
';;;;:l:cWMMMx;;;;;;;;;;;;xMMMWc;;;;;;;'
;;;lNMMMXdMMk;;;;;;;;;;;;;;xNXXk;;;;;;;;
;;;cXMMMXdMMk;;;;;;;;;;;;;;kNNNk;;;;;;;;
';;;;::;cWMMMk:;;;;;;;;;;:kMMMWc;;;;;;;'
 ;;;;;;;;cXMMMWoll;;;;lx0WMMMXc;;;;;;;; 
  ,;;;;;;;;o0NdkMMMMMMMW0KKKx;;;;;;;;,  
   ';;;;;;;;;;:xO0KKKK0lNMMMMk;;;;;;'   
     ';;;;;;;;;;;;;;;;;;dKNN0c;;;;'     
       .',;;;;;;;;;;;;;;;;;;;;,'.       
           ..',,;;;;;;;;,,'..                 
    */

    if line == 0 {
        return "                                        ".bold().truecolor(233, 84, 32)
    } else if line == 1 {
        return "           ..',,;;;;;;;;;,,'..           ".bold().truecolor(233, 84, 32)
    } else if line == 2 {
        return "       .';;;;;;;;;;;;;;;;;;;;;;'.       ".bold().truecolor(233, 84, 32)
    } else if line == 3 {
        return format!("     ';;;;;;;;;;;;;;;;;;{};;;;'     ", "xXNNKc".white()).bold().truecolor(233, 84, 32)
    } else if line == 4 {
        return format!("   ';;;;;;;;;;:{};;;;;;'   ", "xOKXXXXKoNMMMMx".white()).bold().truecolor(233, 84, 32)
    } else if line == 5 {
        return format!(" .;;;;;;;;;{};;;;;;;;;. ", "oKNxxMMMWWMMMKKKKx".white()).bold().truecolor(233, 84, 32)
    } else if line == 6 {
        return format!(" ;;;;;;;;{};;;;{};;;;;;;; ", "lXMMMNocc".white(), "cd0NMMMXl".white()).bold().truecolor(233, 84, 32)
    } else if line == 7 {
        return format!("';;;;::{};;;;;;;;;;;;{};;;;;;;'", "lcWMMMx".white(), "xMMMWc".white()).bold().truecolor(233, 84, 32)
    } else if line == 8 {
        return format!(";;;{};;;;;;;;;;;;;;{};;;;;;;;", "lNMMMXdMMk".white(), "xNXXk".white()).bold().truecolor(233, 84, 32)
    } else if line == 9 {
        return format!(";;;{};;;;;;;;;;;;;;{};;;;;;;;", "cXMMMXdMMk".white(), "kNNNk".white()).bold().truecolor(233, 84, 32)
    } else if line == 10 {
        return format!("';;;;::;{}:;;;;;;;;;;:{};;;;;;;'", "cWMMMk".white(), "kMMMWc".white()).bold().truecolor(233, 84, 32)
    } else if line == 11 {
        return format!(" ;;;;;;;;{};;;;{};;;;;;;; ", "cXMMMWoll".white(), "lx0WMMMXc".white()).bold().truecolor(233, 84, 32)
    } else if line == 12 {
        return format!("  ,;;;;;;;;{};;;;;;;;;,  ", "o0NdkMMMMMMMW0KKKx".white()).bold().truecolor(233, 84, 32)
    } else if line == 13 {
        return format!("   ';;;;;;;;;;:{};;;;;;;'  ", "xO0KKKK0lNMMMMk".white()).bold().truecolor(233, 84, 32);
    } else if line == 14 {
        return format!("     ';;;;;;;;;;;;;;;;;;{};;;;;;'   ", "dKNN0c".white()).bold().truecolor(233, 84, 32);
    } else if line == 15 {
        return "       .',;;;;;;;;;;;;;;;;;;;;;,'.      ".bold().truecolor(233, 84, 32)
    } else if line == 16 {
        return "           ..',,;;;;;;;;;;,,'..           ".bold().truecolor(233, 84, 32)
    } else if line == 17 {
        return "                                        ".bold().truecolor(233, 84, 32)
    } else {
        "?".white()
    }
}

pub fn unknown() -> ColoredString {
    return "?".black()
}