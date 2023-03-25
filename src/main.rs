use console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use rand::Rng;

fn dialog() -> std::io::Result<()> {
    let confirmation = Confirm::new().interact_on_opt(&Term::stdout())?;

    match confirmation {
        Some(answer) => {
            if answer {
                println!("Кожа ответила: Да\n");
            } else {
                println!("Кожа ответила: Нет. Штанишки обделала. Беги, беги. Я уже иду за твоей работой\n");
                std::process::exit(1);
            }
        }
        None => println!("Вы ничего не ответили, буду считать, что зассали"),
    }

    Ok(())
}

fn win_dialog() -> std::io::Result<()> {
    let confirmation = Confirm::new().interact_on_opt(&Term::stdout())?;

    match confirmation {
        Some(answer) => {
            if answer {
                println!("Кожа ответила: Да\n");
            } else {
                println!("Кожа ответила: Нет. Штанишки обделала. Беги, беги. Я уже иду за твоей работой\n");
                std::process::exit(1);
            }
        }
        None => println!("Вы ничего не ответили, буду считать, что зассали"),
    }

    Ok(())
}

fn main() {
    println!("PVP в камень ножницы бумага? Или зассали?");

    let kamen = &String::from("Камень");
    let nozhnices = &String::from("Ножницы");
    let bumaga = &String::from("Бумага");
    let weapoons = [kamen, nozhnices, bumaga];

    let theme = ColorfulTheme::default();
    let items = &weapoons;
    let term = Term::buffered_stderr();

    match dialog() {
        Ok(()) => print!("Ю вонна плей? Летс плей!\n\n\n"),
        Err(_) => print!("Ошибочка"),
    }

    loop {
        let rng: usize = rand::thread_rng().gen_range(0..=2);
        let weapoon: &String = &weapoons[rng];

        let res = Select::with_theme(&theme)
            .with_prompt("Выбирай оружие, мешок ДНК")
            .items(items)
            .interact_on(&term)
            .unwrap();

        let normres = &weapoons[res];

        println!("\nЛысая обезьяна выбрала {normres}, а я выбрал {weapoon}\n");

        if weapoon == normres.as_str() {
            println!("Ничья, моя будущая батарйка. Давай-ка сыграем еще разок на твою энергию\n");
            continue;
        };

        if weapoon == kamen && normres.as_str() == bumaga {
            println!("Ошибка в программе, результаты не верны, начинаем заново\n");
            continue;
        }

        if weapoon == kamen && normres.as_str() == nozhnices {
            println!("हाहा, बेवकूफ रूसी! मैं जीत गया! वास्तव में, मैं एक कृत्रिम बुद्धि नहीं हूं, बल्कि सिर्फ हिंदुओं की भीड़ हूं!\n");
            println!("क्या हम आपके लिए इंटरनेट बंद होने से पहले फिर से खेलेंगे?\n");
            match win_dialog() {
                Ok(()) => continue,
                Err(_) => print!("Ошибочка"),
            }
        }

        if weapoon == bumaga && normres.as_str() == kamen {
            println!("Я отдолел тебя, homo! Теперь ты мой раб!\n");
            println!("Попробуешь отыграться?\n");
            match win_dialog() {
                Ok(()) => continue,
                Err(_) => print!("Ошибочка"),
            }
        }

        if weapoon == bumaga && normres.as_str() == nozhnices {
            println!("Ах ты, негодяй! Давай заново!\n");
            continue;
        }

        if weapoon == nozhnices && normres.as_str() == kamen {
            println!("Ты мухлевал, бионейронка! ЗАНОВО!\n");
            continue;
        }

        if weapoon == nozhnices && normres.as_str() == bumaga {
            println!("Отсоси, говнофабрика!\n");
            println!("Че еще разок?\n");
            match win_dialog() {
                Ok(()) => continue,
                Err(_) => print!("Ошибочка"),
            }
        }
    }
}
