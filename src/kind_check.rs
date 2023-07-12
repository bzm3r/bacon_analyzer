use paste::paste;

#[macro_export]
macro_rules! expand_into_kind_check {
    ($ev_kind:ident ; [ ($possibility:tt),* ]) => {

    }
}

// kind_is_any_of!(event.kind; match self {
//     RaReadError::FileNotFound => [Create(File)],
//     RaReadError::PermissionDenied => [Access(Close(Any), Other, Any)],
//     RaReadError::EmptyFile => [Create(File), Modify(DataChange(_)), Access(Close(Write, Any), Other, Any)],
//     RaReadError::Write(_) | RaReadError::Other(_) | RaReadError::Debouncer(_) => false,
// })

// use std::write;
// use paste::paste;

// macro_rules! final_expand {
//     (   
//         $ev_kind_variant:ident $(; 
//             $inner_enum:ident; 
//             $inner_variant:ident $(; 
//                 $mode_enum_or_variant:ident $(;
//                     $mode_variant:ident
//                 )?
//             )?
//         )?
//     ) => {
//         stringify!(
//             notify::event::EventKind::$ev_kind_variant$(
//             (
//                     notify::event::$inner_enum::$inner_variant $(
//                         (
//                             notify::event::$mode_enum_or_variant $(
//                                 ( notify::event::
//                                     $mode_enum_or_variant::
//                                         $mode_variant
//                                 )
//                             )?
//                         )
//                     )?
//                 )
//             )?
//         )
//     }
// }

// macro_rules! special_mode! {
//     ( 
//         $ev_kind_variant:ident ;
//         $inner_kind_enum:ident ;
//         Data ; 
//         $($mode_variant:ident),+ 
//     ) => {
//         $(
//             final_expand! {
//                 $ev_kind_variant:ident ;
//                 $inner_kind_enum:ident ;
//                 Data ;
//                 DataChange ;
//                 $mode_variant
//             }
//         )|+
//     };
//     ( 
//         $ev_kind_variant:ident ;
//         $inner_kind_enum:ident ;
//         Metadata ; 
//         $($mode_variant:ident),+ 
//     ) => {
//         $(
//             final_expand! {
//                 $ev_kind_variant:ident ;
//                 $inner_kind_enum:ident ;
//                 Metadata ;
//                 MetadataKind ;
//                 $mode_variant
//             }
//         )|+
//     };
// }

// macro_rules! expand_with_mode {
//     (
//         $ev_kind_variant:ident ;
//         $inner_kind_enum:ident ;
//         Data ;
//         $mode_enum:ident ;
//             ( $($mode_variant:ident),+ )
//     ) => {
//         special_mode! {
//             $inner_kind_enum:ident ;
//             Data ;
//             ( $($mode_variant:ident),+ )
//         }
//     };
//         (
//         $ev_kind_variant:ident ;
//         $inner_kind_enum:ident ;
//         Rename ;
//         $mode_enum:ident ;
//             ( $($mode_variant:ident),+ )
//     ) => {
//         special_mode! {
//             $inner_kind_enum:ident ;
//             Metadata ;
//             ( $($mode_variant:ident),+ )
//         }
//     };
//     (
//         $ev_kind_variant:ident ;
//         $inner_kind_enum:ident ;
//         $inner_kind_variant:ident ;
//         $mode_enum:ident ;
//             ( $($mode_variant:ident),+ )
//     ) => {
//         $(
//             final_expand! { 
//                 $ev_kind_variant ;
//                 $inner_kind_enum ;
//                 $inner_kind_variant ;
//                 [< $mode_enum Mode >] ;
//                 $mode_variant
//             }
//         )|+
//     };
//     (
//         $ev_kind_variant:ident ;
//         $inner_kind_enum:ident ;
//         $inner_kind_variant:ident ;
//         $mode_variant:ident
//     ) => {
//         final_expand! {
//             $ev_kind_variant ;
//             $inner_kind_enum ;
//             $inner_kind_variant ;
//             $mode_variant
//         }
//     };
// }

// macro_rules! expand_with_inner_kind {
//     (
//         $ev_kind_variant:ident ;
//         $inner_kind_enum:ident ;
//         $inner_kind_variant:ident ;
//         $( 
//             $mode_enum_or_variant:ident $(
//                 ( $mode_variants:tt )
//             )? 
//         ),+
//     ) => {
//         $(
//             expand_with_mode! {
//                 $ev_kind_variant ;
//                 $inner_kind_enum ;
//                 $inner_kind_variant ;
//                 $mode_enum_or_variant $(; 
//                     $mode_variants
//                 )?
//             }
//         )|+
//     };
//     (
//         $ev_kind_variant:ident ;
//         $inner_kind_enum:ident ;
//         $inner_kind_variant:ident
//     ) => {
//         final_expand! {
//             $ev_kind_variant ;
//             $inner_kind_enum ;
//             $inner_kind_variant
//         }
//     };
// }

// macro_rules! expand_ev_kind {
//     (
//         $ev_kind_variant:ident ;
//             $(
//                 $inner_kind_variant:ident $(
//                     ( $mode:tt )
//                 )?
//             ),+
//     ) => {
//         paste! {
//             $(
//                 expand_with_inner_kind! { 
//                     $ev_kind_variant ;
//                     [<$ev_kind_variant Kind>] ;
//                     $inner_kind_variant $(;
//                         $mode
//                     )?
//                 }
//             )|+
//         }
//     };
//     ( $ev_kind_variant:ident ) => {
//         final_expand! { $ev_kind_variant }
//     };
// }

// macro_rules! expand_ev_kinds {
//     (   $( 
//             $ev_kind_variant:ident $( 
//                 ( $inner_kinds:tt ) 
//             )?
//         ),+ 
//     ) => {
//         [$( 
//             expand_ev_kind! {
//                 $ev_kind_variant $(;
//                     $inner_kinds 
//                 )?
//             }
//         ),+]
//     }
// }

// macro_rules! playground {
//     ($scrutinee:ident ; [ $(ev_kind:tt),* ]) => {
//         match $scrutinee {
//             $(expand_ev_kind!(ev_kind))|*  => true,
//             _ => false,
//         }
//     }
// }

// fn main() {
//     // match self {
//     //     RaReadError::FileNotFound => [Create(File)],
//     //     RaReadError::PermissionDenied => [Access(Close(Any), Other, Any)],
//     //     RaReadError::EmptyFile => [
//     //         Create(File),
//     //         Modify(DataChange(_)),
//     //         Access(Close(Write, Any), Other, Any)
//     //     ],
//     //     RaReadError::Write(_) | RaReadError::Other(_) | RaReadError::Debouncer(_) => false,
//     // }
//     println!("{}", expand_ev_kinds!()));
// }
