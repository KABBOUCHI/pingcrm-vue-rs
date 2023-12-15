use glob::glob;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;
use std::{env, fs};

#[proc_macro]
pub fn migrations(_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let glob_path = format!(
        "{}/{}/*.rs",
        env::var("CARGO_MANIFEST_DIR").unwrap(),
        "migrations"
    );

    let mut files = vec![];

    let paths = glob(&glob_path)
        .expect("Failed to read glob pattern")
        .flatten();

    let re = Regex::new(r"impl\s+Migration\s+for\s+(\w+)").unwrap();

    for path in paths {
        let file_name = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .replace(".rs", "");

        let file_contents = fs::read_to_string(&path).unwrap();
        let capture = re.captures(&file_contents);

        if let Some(capture) = capture {
            let struc = capture.get(1).unwrap().as_str().to_string(); // Clone the captured value
            files.push((file_name, struc));
        }
    }

    let mut mods = TokenStream::new();
    let mut items = TokenStream::new();

    files.iter().for_each(|(file, struc)| {
        let a = format_ident!("{}", file);
        let b = format_ident!("{}", struc);

        let token: TokenStream = quote!((#file.to_string(), Box::new(#a::#b)),);

        items.extend(token);

        let token: TokenStream = quote!(mod #a;);

        mods.extend(token);
    });

    quote! {
        use std::collections::HashMap;
        use anyhow::Ok;
        use ensemble::{
            migrations::{Migration, Migrator},
            query::Builder,
        };

        #mods

         async fn migrator() -> anyhow::Result<Migrator> {
            let mut migrator = Migrator::new().await?;

            let migrations: Vec<(String, Box<dyn Migration>)> = vec![#items];

            for migration in migrations {
                migrator.register(migration.0, migration.1);
            }

            Ok(migrator)
        }

        pub async fn wipe() -> anyhow::Result<()> {
            // let tables =  unsafe { Builder::raw_sql("SELECT TABLE_NAME as name FROM information_schema.TABLES WHERE TABLE_SCHEMA = DATABASE()", vec![]) }.await?;

            // let tables = tables
            //     .into_iter()
            //     .map(rbs::from_value::<HashMap<String, String>>);

            // for table in tables {
            //     let table = table?;

            //     if let Some(name) = table.get("name") {
            //         println!("Dropping table {}", name);

            //         unsafe {
            //             // Builder::raw_sql("DROP TABLE IF EXISTS ?", vec![ensemble::value::for_db(name)?]).await?;
            //             Builder::raw_sql(&format!("DROP TABLE IF EXISTS {}", name), vec![]).await?;
            //         }
            //     }
            // }

            Ok(())
        }

        pub async fn fresh() -> anyhow::Result<()> {
            // wipe().await?;
            // migrate().await
            todo!()
        }

        pub async fn migrate() -> anyhow::Result<()> {
            migrator().await?.run().await?;

            println!("Migrated database.");

            Ok(())
        }

        pub async fn rollback(batches: u64) -> anyhow::Result<()> {
            migrator().await?.rollback(batches).await?;

            println!("Rolled back database.");

            Ok(())
        }

        pub async fn status() -> anyhow::Result<()> {
            let m = migrator().await?;
            let pending_migrations = m.pending();
            let store = m.status();

            println!("{:<70} {:<30}", "Migration name", "Batch / Status",);

            for migration in store {
                println!(
                    "{:<70} {:<30}",
                    migration.migration,
                    format!("[{}] Ran", migration.batch),
                );
            }

            for migration in pending_migrations {
                println!("{:<70} {:<30}", migration.0, "Pending".to_string(),);
            }

            Ok(())
        }

    }
    .into()
}
