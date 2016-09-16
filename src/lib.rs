#[macro_use]
/// # FR:
/// 
/// Le pack de loggers contient le module:
/// 
/// - proce
/// - oop
/// 
/// # EN:
/// 
/// The loggers pack contains this mods:
/// 
/// - proce
/// - oop
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
		macro_rules! info {
			($message_content:expr) => (log!("[INFO]:", $message_content));
			($message_content:expr, $variable:expr) => (log!("[INFO]:", $message_content, $variable));
			($struct_name:expr, $message_content:expr, $variable:expr) => (log!($struct_name, "[INFO]:", $message_content, $variable));
		}
		#[macro_export]
		macro_rules! warn {
			($message_content:expr) => (log!("[WARNING]:", $message_content));
			($message_content:expr, $variable:expr) => (log!("[WARNING]:", $message_content, $variable));
			($struct_name:expr, $message_content:expr, $variable:expr) => (log!($struct_name, "[WARNING]:", $message_content, $variable));
		}
		#[macro_export]
		macro_rules! error {
			($message_content:expr) => (log!("[ERROR]:", $message_content));
			($message_content:expr, $variable:expr) => (log!("[ERROR]:", $message_content, $variable));
			($struct_name:expr, $message_content:expr, $variable:expr) => (log!($struct_name, "[ERROR]:", $message_content, $variable));
		}
		#[macro_export]
		macro_rules! success {
			($message_content:expr) => (log!("[SUCCESS]:", $message_content));
			($message_content:expr, $variable:expr) => (log!("[SUCCESS]:", $message_content, $variable));
			($struct_name:expr, $message_content:expr, $variable:expr) => (log!($struct_name, "[SUCCESS]:", $message_content, $variable));
		}
	}

    /// Contient les outils orientés objet
    /// Contains OO(object oriented) tools
    /// 
    /// ----
    /// 
    /// Contiendra les *wrappers* de chaque macros encapsulées par le module `loggers_pack::proce`.
    /// 
    /// ### Attention:
    /// 
    /// Les *wrappers* (fonctions) supporteront l'intégralité des paramètres que peuvent capturer les macros;
    /// Vous devrez donc veillez à laisser une chaîne de caractères vide là où vous ne souhaitez pas renseigner d'informations.
    /// 
    /// Exemple:
    /// 
    /// ```Rust 
    /// 
    /// struct A;
    /// 
    /// use loggers_pack::oop::Logger;
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
		/// Utilise 
		pub trait Logger
		{
			fn info()
			{

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

}