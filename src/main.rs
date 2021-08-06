use structopt::StructOpt;
use anyhow::{Result, bail};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(StructOpt)]
struct Opt {

    // short and long flags (-t, --temp) will be deduced from the field's name
    #[structopt(short, long)]
    temp: f32,
    
    // short and long flags (-h, --humid) will be deduced from the field's name
    #[structopt(short, long)]
    humid: f32,

    // short and long flags (-i, --input) will be deduced from the field's name
    #[structopt(short, long, default_value = "ºC")]
    input: String
}

fn far_to_celc(temp_f:f32)->f32 {
    (temp_f-32.0)/1.8
}

fn celc_to_far(temp_c:f32)->f32 {
    (temp_c*1.8)+32.0
}

fn calc_wetbulb(temp_c:f32, humid:f32)->f32 {
    temp_c*(0.151977*(humid+8.313659).powf(1.0/2.0)).atan()+(temp_c + humid).atan()-(humid-1.676331).atan()
    +0.00391838*humid.powf(3.0/2.0)*(0.023101*humid).atan()-4.686035
}

fn calc_heat_index(temp_f:f32, humid:f32)->f32 {
    let hi_temp = 61.0+((temp_f-68.0)*1.2)+(humid*0.094);
    let hi_final = 0.5*(temp_f+hi_temp);
    let mut hi = hi_final;

    if hi_final > 79.0 {
        hi = -42.379+2.04901523*temp_f+10.14333127*humid-0.22475541*temp_f*humid-6.83783*((10.0f32).powf(-3.0))
        *(temp_f.powf(2.0))-5.481717*((10.0f32).powf(-2.0))*(humid.powf(2.0))+1.22874*((10.0f32).powf(-3.0))
        *(temp_f.powf(2.0))*humid+8.5282*((10.0f32).powf(-4.0))*temp_f*(humid.powf(2.0))-1.99*((10.0f32).powf(-6.0))
        *(temp_f.powf(2.0))*(humid.powf(2.0))
    }
    else if humid > 85.0 && temp_f >= 80.0 && temp_f <= 87.0 {
        hi = hi + (((humid-85.0)/10.0) + ((87.0-temp_f)/5.0));
    } 

    return hi;
}

fn calc_dew_point(temp_c:f32, humid:f32)->f32 {
    let h = ((humid).log10()-2.0)/0.4343 + (17.62*temp_c)/(243.12+temp_c); 
    return 243.12*h/(17.62-h);
}

fn main() -> Result<()> {

    let opt = Opt::from_args();

    let temp_c;
    let temp_f;
    let input = opt.input;

    if input != "ºC" && input != "ºF" {
        bail!("Input unit '{}' is neither ºC nor ºF", input);
    }
    
    if "ºC" == input {
        temp_c = opt.temp; 
    }
    else {
        temp_c = far_to_celc(opt.temp); 
    };

    if "ºF" == input {
        temp_f = opt.temp; 
    }
    else {
        temp_f = celc_to_far(opt.temp); 
    };

    let wet_bulb_c = calc_wetbulb(temp_c, opt.humid);
    let wet_bulb_f = celc_to_far(wet_bulb_c);
    let heat_index_f = calc_heat_index(temp_f, opt.humid);
    let dew_point_c = calc_dew_point(temp_c, opt.humid);
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let textwbt = format!("wet-bulb temperature: {:.1}ºC, {:.1}ºF", wet_bulb_c, wet_bulb_f);
    let texthi = format!("heat index: {:.1}ºC, {:.1}ºF", far_to_celc(heat_index_f), heat_index_f);
    let textdp = format!("dew point: {:.1}ºC, {:.1}ºF", dew_point_c, celc_to_far(dew_point_c));

    if wet_bulb_f <= 80.0 {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    }
    else if wet_bulb_f <= 85.0 {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    }
    else if wet_bulb_f <= 88.0 {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    }
    else if wet_bulb_f <= 90.0 {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
    }
    else if wet_bulb_f > 90.0 {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
    }

    println!("{}", textwbt);
    println!("{}", texthi);
    println!("{}", textdp);

    stdout.reset()?;

    Ok(())
}
