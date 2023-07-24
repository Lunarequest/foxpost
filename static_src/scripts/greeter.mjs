import figlet from "figlet";
import chalk from "chalk";


let logo_color = chalk.hex("#f6b558");
console.log(logo_color(figlet.textSync("FOX POST")));
