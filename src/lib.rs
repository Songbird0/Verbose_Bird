#[macro_use]
/// # FR:
/// 
/// Le pack de loggers contient le module:
/// 
/// - [`proce`]
/// - [`oop`]
/// 
/// [`proce`]: proce/
/// [`oop`]: oop/
/// 
/// # EN:
/// 
/// The loggers pack contains this mods:
/// 
/// - [`proce`]
/// - [`oop`]
/// 
pub mod loggers_pack
{
	#[macro_use]
	/// Contient les outils procéduraux.
	/// La classe [Duration](https://doc.rust-lang.org/1.0.0/std/time/duration/struct.Duration.html) étant, à ce jour (15 septembre 2016), instable
	/// il est préférable de ne pas l'implémenter.
	pub mod proce
	{
		#[macro_export]
		/// # FR:
		/// 
		/// La macro `log!` est une amorçe permettant d'encapsuler des manipulations inutiles (concaténation manuelle).
		/// log! amorce les macros suivantes:
		/// 
		/// - [`info!()`]
		/// - [`warn!()`]
		/// - [`error!()`]
		/// - [`success!()`]
		/// 
		/// 
		/// [`info!()`]: macro.info!.html
		/// [`warn!()`]: macro.warn!.html
		/// [`error!()`]: macro.error!.html
		/// [`success!()`]: macro.success!.html
		/// 
		/// Ces dernières servent elles-même d'amorçes pour les fonctions du package `loggers_pack::oop`.
		macro_rules! log {

			($struct_name:expr, $log_type:expr, $message_content:expr, $variable:expr) => 
			{
				let mut content : String = String::new();
				content.push_str($struct_name);
				content.push_str($log_type);
				content.push_str($message_content);
				content.push_str($variable);
				println!("{}", &content);
			};

			($log_type:expr, $message_content:expr, $variable:expr) => 
			{
				log!("", $log_type, $message_content, $variable);
			};

			($log_type:expr, $message_content:expr) => 
			{
				log!("",$log_type, $message_content, "");
			};
		}
		#[macro_export]
		/// #FR:
		/// ## Dépendences:
		/// 
		/// - [`log!()`]
		/// 
		/// Exemple d'utilisation:
		/// 
		/// ```Rust
		/// 
		/// info!("Hello world! :D"); // => [INFO]: Hello world! :D
		/// info!("Hello ", "world! :D"); // => [INFO]: Hello world! :D
		/// info!("@StructName", "Hello ", "world! :D"); // => @StructName[INFO]: Hello world! :D
		/// 
		/// ```
		/// 
		/// [`log!()`]: macro.log!.html  
		macro_rules! info {
			($message_content:expr) => (log!("[INFO]:", $message_content));
			($message_content:expr, $variable:expr) => (log!("[INFO]:", $message_content, $variable));
			($struct_name:expr, $message_content:expr, $variable:expr) => (log!($struct_name, "[INFO]:", $message_content, $variable));
		}
		#[macro_export]
		/// #FR:
		/// ## Dépendences:
		/// 
		/// - [`log!()`]
		/// 
		/// Exemple d'utilisation:
		/// 
		/// ```Rust
		/// 
		/// warn!("Hello world! :D"); // => [WARNING]: Hello world! :D
		/// warn!("Hello ", "world! :D"); // => [WARNING]: Hello world! :D
		/// warn!("@StructName", "Hello ", "world! :D"); // => @StructName[WARNING]: Hello world! :D
		/// 
		/// ```
		/// 
		/// [`log!()`]: macro.log!.html  
		macro_rules! warn {
			($message_content:expr) => (log!("[WARNING]:", $message_content));
			($message_content:expr, $variable:expr) => (log!("[WARNING]:", $message_content, $variable));
			($struct_name:expr, $message_content:expr, $variable:expr) => (log!($struct_name, "[WARNING]:", $message_content, $variable));
		}
		#[macro_export]
		/// #FR:
		/// ## Dépendences:
		/// 
		/// - [`log!()`]
		/// 
		/// Exemple d'utilisation:
		/// 
		/// ```Rust
		/// 
		/// error!("Hello world! :D"); // => [ERROR]: Hello world! :D
		/// error!("Hello ", "world! :D"); // => [ERROR]: Hello world! :D
		/// error!("@StructName", "Hello ", "world! :D"); // => @StructName[ERROR]: Hello world! :D
		/// 
		/// ```
		/// 
		/// [`log!()`]: macro.log!.html  
		macro_rules! error {
			($message_content:expr) => (log!("[ERROR]:", $message_content));
			($message_content:expr, $variable:expr) => (log!("[ERROR]:", $message_content, $variable));
			($struct_name:expr, $message_content:expr, $variable:expr) => (log!($struct_name, "[ERROR]:", $message_content, $variable));
		}
		#[macro_export]
		/// #FR:
		/// ## Dépendences:
		/// 
		/// - [`log!()`]
		/// 
		/// Exemple d'utilisation:
		/// 
		/// ```Rust
		/// 
		/// success!("Hello world! :D"); // => [SUCCESS]: Hello world! :D
		/// success!("Hello ", "world! :D"); // => [SUCCESS]: Hello world! :D
		/// success!("@StructName", "Hello ", "world! :D"); // => @StructName[SUCCESS]: Hello world! :D
		/// 
		/// ```
		/// 
		/// [`log!()`]: macro.log!.html  
		macro_rules! success {
			($message_content:expr) => (log!("[SUCCESS]:", $message_content));
			($message_content:expr, $variable:expr) => (log!("[SUCCESS]:", $message_content, $variable));
			($struct_name:expr, $message_content:expr, $variable:expr) => (log!($struct_name, "[SUCCESS]:", $message_content, $variable));
		}
	}

    /// Contient les outils orientés objet
    /// 
    /// Contains OO(object oriented) tools
    /// 
    /// ----
    /// 
    /// Contiendra les *wrappers* de chaque macros encapsulées par le module `loggers_pack::proce`.
    /// 
    /// ### Attention:
    /// 
    /// Les *wrappers* (fonctions) supporteront l'intégralité des paramètres que peuvent capturer les macros;
    /// Vous devrez donc veiller à laisser une chaîne de caractères vide là où vous ne souhaitez pas renseigner d'informations.
    /// 
    /// Exemple:
    /// 
    /// ``` 
    /// struct A;
    /// 
    /// use loggers_pack::oop::*;
    /// 
    /// impl Logger for A 
    /// {/* Les fonctions sont implémentées */}
    /// 
    /// let foo : A = A;
    /// foo.info("source du message","contenu du message", "chaîne de caractères à concaténer si besoin");
    /// //si vous ne souhaitez pas forcément remplir chaque paramètre:
    /// 
    /// foo.info("", "contenu du message", ""); //ça suffit
    /// 
    /// ```
	pub mod oop
	{
		/// Chaque fonction a été volontairement écrite de manière à être *indépendante* de chaque instance
		/// de la structure.
		/// Cela permettra de pouvoir dégager rapidement les informations que pourraient renvoyer les instances de cette dernière. 
		pub trait Logger
		{
			/// # info
			/// 
			fn info(struct_name: &str, message_content: &str, variable: &str) -> ()
			{
				info!(struct_name, message_content, variable);
			}

			fn warn(struct_name: &str, message_content: &str, variable: &str) -> ()
			{
				warn!(struct_name, message_content, variable);
			}

			fn error(struct_name: &str, message_content: &str, variable: &str) -> ()
			{
				error!(struct_name, message_content, variable);
			}

			fn success(struct_name: &str, message_content: &str, variable: &str) -> ()
			{
				success!(struct_name, message_content, variable);
			}
		}

	}
}

#[cfg(test)]
mod proce_tests
{
	#[test]
	fn pack_logger_proce_custom()
	{
		log!("[CUSTOM]:", "CUSTOM log is OK. (run with two args)");
		log!("[CUSTOM]:", "CUSTOM log is ", "OK. (run with three args)");
		log!("[CUSTOM_STRUCT_NAME]","[CUSTOM]:", "CUSTOM log is ", "OK. (run with three args)");
	}

	#[test]
	fn pack_logger_proce_info()
	{
		info!("INFO log is OK. (run with one arg)");
		info!("INFO log is OK.", " (run with two args)");
		info!("[INFO_STRUCT_NAME]","INFO log is ", "OK. (run with three args)");
	}

	#[test]
	fn pack_logger_proce_warn()
	{
		warn!("WARNING log is OK. (run with one arg)");
		warn!("WARNING log is OK. ", "(run with two args)");
		warn!("[WARNING_STRUCT_NAME]","WARNING log is ", "OK. (run with three args)");
	}

	#[test]
	fn pack_logger_proce_error()
	{
		error!("ERROR log is OK. (run with one arg)");
		error!("ERROR log is OK. ", "(run with two args)");
		error!("[ERROR_STRUCT_NAME]","ERROR log is ", "OK. (run with three args)");
	}

	#[test]
	fn pack_logger_proce_success()
	{
		success!("SUCCESS log is OK. (run with one arg)");
		success!("SUCCESS log is OK. ", "(run with two args)");
		success!("[SUCCESS_STRUCT_NAME]","SUCCESS log is ", "OK. (run with three args)");
	}
}

#[cfg(test)]
mod oo_tests
{
	struct Alice;
	use loggers_pack::oop::Logger;
	impl Logger for Alice{/*...*/}

	#[test]
	fn pack_logger_oop_info()
	{
		Alice::info("@Alice", "Hello, I'm Alice ", "Peterson !");
	}

	#[test]
	fn pack_logger_oop_wan()
	{
		Alice::warn("@Alice", "Hello, I'm Alice ", "Peterson !");
	}

	#[test]
	fn pack_logger_oop_error()
	{
		Alice::error("@Alice", "Hello, I'm Alice ", "Peterson !");
	}

	#[test]
	fn pack_logger_oop_success()
	{
		Alice::success("@Alice", "Hello, I'm Alice ", "Peterson !");
	}
}
