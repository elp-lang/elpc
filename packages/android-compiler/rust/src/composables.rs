use jni::*;
use serde_json;
use std::ptr;

use crate::types::{self, get_jvm, Unit};

#[no_mangle]
pub extern "system" fn Java_com_elp_Label_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, label_name: JString<'local>, label_args: JString<'local>, modifier: Modifier, interactionSource: MutableInteractionSource, isPersistent: Boolean, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let Label_composable = if Label_args.is_null() {
		Composable::new(env.get_string(Label_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Label_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Label_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Text_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, text: String, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, minLines: Int, onTextLayout: KotlinCallback::new(vec!("TextLayoutResult"), "Unit"), style: TextStyle) -> jobject {
	let jvm = get_jvm(env);
	let Text_composable = if Text_args.is_null() {
		Composable::new(env.get_string(Text_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Text_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Text_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Text_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, text: String, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, onTextLayout: (TextLayoutResult) -> Unit, style: TextStyle) -> jobject {
	let jvm = get_jvm(env);
	let Text_composable = if Text_args.is_null() {
		Composable::new(env.get_string(Text_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Text_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Text_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Text_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, text: AnnotatedString, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, minLines: Int, inlineContent: Map<String, InlineTextContent>, onTextLayout: (TextLayoutResult) -> Unit, style: TextStyle) -> jobject {
	let jvm = get_jvm(env);
	let Text_composable = if Text_args.is_null() {
		Composable::new(env.get_string(Text_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Text_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Text_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Text_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, text: AnnotatedString, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, inlineContent: Map<String, InlineTextContent>, onTextLayout: (TextLayoutResult) -> Unit, style: TextStyle) -> jobject {
	let jvm = get_jvm(env);
	let Text_composable = if Text_args.is_null() {
		Composable::new(env.get_string(Text_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Text_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Text_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ProvideTextStyle_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: TextStyle, content: @Composable () -> Unit) -> jobject {
	let jvm = get_jvm(env);
	let ProvideTextStyle_composable = if ProvideTextStyle_args.is_null() {
		Composable::new(env.get_string(ProvideTextStyle_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(ProvideTextStyle_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &ProvideTextStyle_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_DatePicker_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: DatePickerState, modifier: Modifier, dateFormatter: DatePickerFormatter, title: (@Composable () -> Unit) -> jobject {
	let jvm = get_jvm(env);
	let DatePicker_composable = if DatePicker_args.is_null() {
		Composable::new(env.get_string(DatePicker_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(DatePicker_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &DatePicker_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, titleContentColor: Color, headlineContentColor: Color, weekdayContentColor: Color, subheadContentColor: Color, navigationContentColor: Color, yearContentColor: Color, disabledYearContentColor: Color, currentYearContentColor: Color, selectedYearContentColor: Color, disabledSelectedYearContentColor: Color, selectedYearContainerColor: Color, disabledSelectedYearContainerColor: Color, dayContentColor: Color, disabledDayContentColor: Color, selectedDayContentColor: Color, disabledSelectedDayContentColor: Color, selectedDayContainerColor: Color, disabledSelectedDayContainerColor: Color, todayContentColor: Color, todayDateBorderColor: Color, dayInSelectionRangeContentColor: Color, dayInSelectionRangeContainerColor: Color, dividerColor: Color, dateTextFieldColors: TextFieldColors) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_DatePickerTitle_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, displayMode: DisplayMode, modifier: Modifier) -> jobject {
	let jvm = get_jvm(env);
	let DatePickerTitle_composable = if DatePickerTitle_args.is_null() {
		Composable::new(env.get_string(DatePickerTitle_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(DatePickerTitle_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &DatePickerTitle_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Transition_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, inputState: InputPhase, focusedTextStyleColor: Color, unfocusedTextStyleColor: Color, contentColor_name: JString<'local>, contentColor_args: JString<'local>, showLabel: Boolean, content_name: JString<'local>, content_args: JString<'local>, labelProgress: Float, labelTextStyleColor: Color, labelContentColor: Color, placeholderOpacity: Float, prefixSuffixOpacity: Float) -> jobject {
	let jvm = get_jvm(env);
	let Transition_composable = if Transition_args.is_null() {
		Composable::new(env.get_string(Transition_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Transition_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Transition_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Scaffold_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, topBar_name: JString<'local>, topBar_args: JString<'local>, bottomBar_name: JString<'local>, bottomBar_args: JString<'local>, snackbarHost_name: JString<'local>, snackbarHost_args: JString<'local>, floatingActionButton_name: JString<'local>, floatingActionButton_args: JString<'local>, floatingActionButtonPosition: FabPosition, containerColor: Color, contentColor: Color, contentWindowInsets: WindowInsets, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let Scaffold_composable = if Scaffold_args.is_null() {
		Composable::new(env.get_string(Scaffold_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Scaffold_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Scaffold_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, clockDialColor: Color, clockDialSelectedContentColor: Color, clockDialUnselectedContentColor: Color, selectorColor: Color, containerColor: Color, periodSelectorBorderColor: Color, periodSelectorSelectedContainerColor: Color, periodSelectorUnselectedContainerColor: Color, periodSelectorSelectedContentColor: Color, periodSelectorUnselectedContentColor: Color, timeSelectorSelectedContainerColor: Color, timeSelectorUnselectedContainerColor: Color, timeSelectorSelectedContentColor: Color, timeSelectorUnselectedContentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_layoutType_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let layoutType_composable = if layoutType_args.is_null() {
		Composable::new(env.get_string(layoutType_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(layoutType_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &layoutType_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_itemColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let itemColors_composable = if itemColors_args.is_null() {
		Composable::new(env.get_string(itemColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(itemColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &itemColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_itemColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, textColor: Color, leadingIconColor: Color, trailingIconColor: Color, disabledTextColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let itemColors_composable = if itemColors_args.is_null() {
		Composable::new(env.get_string(itemColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(itemColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &itemColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Slider_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: Float, onValueChange: (Float) -> jobject {
	let jvm = get_jvm(env);
	let Slider_composable = if Slider_args.is_null() {
		Composable::new(env.get_string(Slider_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Slider_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Slider_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_RangeSlider_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: ClosedFloatingPointRange<Float>, onValueChange: (ClosedFloatingPointRange<Float>) -> jobject {
	let jvm = get_jvm(env);
	let RangeSlider_composable = if RangeSlider_args.is_null() {
		Composable::new(env.get_string(RangeSlider_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(RangeSlider_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &RangeSlider_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, thumbColor: Color, activeTrackColor: Color, activeTickColor: Color, inactiveTrackColor: Color, inactiveTickColor: Color, disabledThumbColor: Color, disabledActiveTrackColor: Color, disabledActiveTickColor: Color, disabledInactiveTrackColor: Color, disabledInactiveTickColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Thumb_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, interactionSource: MutableInteractionSource, modifier: Modifier, colors: SliderColors, enabled: Boolean, thumbSize: DpSize) -> jobject {
	let jvm = get_jvm(env);
	let Thumb_composable = if Thumb_args.is_null() {
		Composable::new(env.get_string(Thumb_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Thumb_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Thumb_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Track_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, rangeSliderState: RangeSliderState, modifier: Modifier, colors: SliderColors, enabled: Boolean) -> jobject {
	let jvm = get_jvm(env);
	let Track_composable = if Track_args.is_null() {
		Composable::new(env.get_string(Track_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Track_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Track_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Card_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let Card_composable = if Card_args.is_null() {
		Composable::new(env.get_string(Card_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Card_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Card_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Card_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let Card_composable = if Card_args.is_null() {
		Composable::new(env.get_string(Card_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Card_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Card_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ElevatedCard_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, shape: Shape, colors: CardColors, elevation: CardElevation, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let ElevatedCard_composable = if ElevatedCard_args.is_null() {
		Composable::new(env.get_string(ElevatedCard_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(ElevatedCard_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &ElevatedCard_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ElevatedCard_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: CardColors, elevation: CardElevation, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let ElevatedCard_composable = if ElevatedCard_args.is_null() {
		Composable::new(env.get_string(ElevatedCard_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(ElevatedCard_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &ElevatedCard_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedCard_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let OutlinedCard_composable = if OutlinedCard_args.is_null() {
		Composable::new(env.get_string(OutlinedCard_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(OutlinedCard_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &OutlinedCard_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedCard_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let OutlinedCard_composable = if OutlinedCard_args.is_null() {
		Composable::new(env.get_string(OutlinedCard_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(OutlinedCard_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &OutlinedCard_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_cardElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let jvm = get_jvm(env);
	let cardElevation_composable = if cardElevation_args.is_null() {
		Composable::new(env.get_string(cardElevation_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(cardElevation_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &cardElevation_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedCardElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let jvm = get_jvm(env);
	let elevatedCardElevation_composable = if elevatedCardElevation_args.is_null() {
		Composable::new(env.get_string(elevatedCardElevation_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(elevatedCardElevation_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &elevatedCardElevation_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_outlinedCardElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let jvm = get_jvm(env);
	let outlinedCardElevation_composable = if outlinedCardElevation_args.is_null() {
		Composable::new(env.get_string(outlinedCardElevation_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(outlinedCardElevation_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &outlinedCardElevation_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_cardColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let cardColors_composable = if cardColors_args.is_null() {
		Composable::new(env.get_string(cardColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(cardColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &cardColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_cardColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let cardColors_composable = if cardColors_args.is_null() {
		Composable::new(env.get_string(cardColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(cardColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &cardColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedCardColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let elevatedCardColors_composable = if elevatedCardColors_args.is_null() {
		Composable::new(env.get_string(elevatedCardColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(elevatedCardColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &elevatedCardColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedCardColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let elevatedCardColors_composable = if elevatedCardColors_args.is_null() {
		Composable::new(env.get_string(elevatedCardColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(elevatedCardColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &elevatedCardColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_outlinedCardColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let outlinedCardColors_composable = if outlinedCardColors_args.is_null() {
		Composable::new(env.get_string(outlinedCardColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(outlinedCardColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &outlinedCardColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_outlinedCardColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let outlinedCardColors_composable = if outlinedCardColors_args.is_null() {
		Composable::new(env.get_string(outlinedCardColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(outlinedCardColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &outlinedCardColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_outlinedCardBorder_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, enabled: Boolean) -> jobject {
	let jvm = get_jvm(env);
	let outlinedCardBorder_composable = if outlinedCardBorder_args.is_null() {
		Composable::new(env.get_string(outlinedCardBorder_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(outlinedCardBorder_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &outlinedCardBorder_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Button_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let Button_composable = if Button_args.is_null() {
		Composable::new(env.get_string(Button_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Button_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Button_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ElevatedButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let ElevatedButton_composable = if ElevatedButton_args.is_null() {
		Composable::new(env.get_string(ElevatedButton_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(ElevatedButton_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &ElevatedButton_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_FilledTonalButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let FilledTonalButton_composable = if FilledTonalButton_args.is_null() {
		Composable::new(env.get_string(FilledTonalButton_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(FilledTonalButton_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &FilledTonalButton_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let OutlinedButton_composable = if OutlinedButton_args.is_null() {
		Composable::new(env.get_string(OutlinedButton_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(OutlinedButton_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &OutlinedButton_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_TextButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let TextButton_composable = if TextButton_args.is_null() {
		Composable::new(env.get_string(TextButton_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(TextButton_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &TextButton_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_buttonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let buttonColors_composable = if buttonColors_args.is_null() {
		Composable::new(env.get_string(buttonColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(buttonColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &buttonColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_buttonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let buttonColors_composable = if buttonColors_args.is_null() {
		Composable::new(env.get_string(buttonColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(buttonColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &buttonColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let elevatedButtonColors_composable = if elevatedButtonColors_args.is_null() {
		Composable::new(env.get_string(elevatedButtonColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(elevatedButtonColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &elevatedButtonColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let elevatedButtonColors_composable = if elevatedButtonColors_args.is_null() {
		Composable::new(env.get_string(elevatedButtonColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(elevatedButtonColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &elevatedButtonColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_filledTonalButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let filledTonalButtonColors_composable = if filledTonalButtonColors_args.is_null() {
		Composable::new(env.get_string(filledTonalButtonColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(filledTonalButtonColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &filledTonalButtonColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_filledTonalButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let filledTonalButtonColors_composable = if filledTonalButtonColors_args.is_null() {
		Composable::new(env.get_string(filledTonalButtonColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(filledTonalButtonColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &filledTonalButtonColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_outlinedButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let outlinedButtonColors_composable = if outlinedButtonColors_args.is_null() {
		Composable::new(env.get_string(outlinedButtonColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(outlinedButtonColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &outlinedButtonColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_outlinedButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let outlinedButtonColors_composable = if outlinedButtonColors_args.is_null() {
		Composable::new(env.get_string(outlinedButtonColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(outlinedButtonColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &outlinedButtonColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_textButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let textButtonColors_composable = if textButtonColors_args.is_null() {
		Composable::new(env.get_string(textButtonColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(textButtonColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &textButtonColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_buttonElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp) -> jobject {
	let jvm = get_jvm(env);
	let buttonElevation_composable = if buttonElevation_args.is_null() {
		Composable::new(env.get_string(buttonElevation_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(buttonElevation_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &buttonElevation_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedButtonElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp) -> jobject {
	let jvm = get_jvm(env);
	let elevatedButtonElevation_composable = if elevatedButtonElevation_args.is_null() {
		Composable::new(env.get_string(elevatedButtonElevation_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(elevatedButtonElevation_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &elevatedButtonElevation_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_filledTonalButtonElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp) -> jobject {
	let jvm = get_jvm(env);
	let filledTonalButtonElevation_composable = if filledTonalButtonElevation_args.is_null() {
		Composable::new(env.get_string(filledTonalButtonElevation_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(filledTonalButtonElevation_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &filledTonalButtonElevation_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_NavigationBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, containerColor: Color, contentColor: Color, tonalElevation: Dp, windowInsets: WindowInsets, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let NavigationBar_composable = if NavigationBar_args.is_null() {
		Composable::new(env.get_string(NavigationBar_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(NavigationBar_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &NavigationBar_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, disabledIconColor: Color, disabledTextColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_MaterialTheme_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, colorScheme: ColorScheme, shapes: Shapes, typography: Typography, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let MaterialTheme_composable = if MaterialTheme_args.is_null() {
		Composable::new(env.get_string(MaterialTheme_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(MaterialTheme_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &MaterialTheme_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Checkbox_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, checked: Boolean, onCheckedChange: KotlinCallback::new(vec!["Boolean"], "Unit"), modifier: Modifier, enabled: Boolean, colors: CheckboxColors, interactionSource: MutableInteractionSource) -> jobject {
	let jvm = get_jvm(env);
	let Checkbox_composable = if Checkbox_args.is_null() {
		Composable::new(env.get_string(Checkbox_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Checkbox_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Checkbox_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_TriStateCheckbox_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: ToggleableState, onClick: KotlinCallback::new(vec![], "Unit"), modifier: Modifier, enabled: Boolean, colors: CheckboxColors, interactionSource: MutableInteractionSource) -> jobject {
	let jvm = get_jvm(env);
	let TriStateCheckbox_composable = if TriStateCheckbox_args.is_null() {
		Composable::new(env.get_string(TriStateCheckbox_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(TriStateCheckbox_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &TriStateCheckbox_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, checkedColor: Color, uncheckedColor: Color, checkmarkColor: Color, disabledCheckedColor: Color, disabledUncheckedColor: Color, disabledIndeterminateColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_TextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: String, onValueChange: (String) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label_name: JString<'local>, label_args: JString<'local>, placeholder_name: JString<'local>, placeholder_args: JString<'local>, leadingIcon_name: JString<'local>, leadingIcon_args: JString<'local>, trailingIcon_name: JString<'local>, trailingIcon_args: JString<'local>, prefix_name: JString<'local>, prefix_args: JString<'local>, suffix_name: JString<'local>, suffix_args: JString<'local>, supportingText_name: JString<'local>, supportingText_args: JString<'local>, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
	let jvm = get_jvm(env);
	let TextField_composable = if TextField_args.is_null() {
		Composable::new(env.get_string(TextField_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(TextField_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &TextField_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_TextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: TextFieldValue, onValueChange: (TextFieldValue) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label_name: JString<'local>, label_args: JString<'local>, placeholder_name: JString<'local>, placeholder_args: JString<'local>, leadingIcon_name: JString<'local>, leadingIcon_args: JString<'local>, trailingIcon_name: JString<'local>, trailingIcon_args: JString<'local>, prefix_name: JString<'local>, prefix_args: JString<'local>, suffix_name: JString<'local>, suffix_args: JString<'local>, supportingText_name: JString<'local>, supportingText_args: JString<'local>, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
	let jvm = get_jvm(env);
	let TextField_composable = if TextField_args.is_null() {
		Composable::new(env.get_string(TextField_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(TextField_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &TextField_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_TextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: String, onValueChange: (String) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label_name: JString<'local>, label_args: JString<'local>, placeholder_name: JString<'local>, placeholder_args: JString<'local>, leadingIcon_name: JString<'local>, leadingIcon_args: JString<'local>, trailingIcon_name: JString<'local>, trailingIcon_args: JString<'local>, supportingText_name: JString<'local>, supportingText_args: JString<'local>, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
	let jvm = get_jvm(env);
	let TextField_composable = if TextField_args.is_null() {
		Composable::new(env.get_string(TextField_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(TextField_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &TextField_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_TextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: TextFieldValue, onValueChange: (TextFieldValue) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label_name: JString<'local>, label_args: JString<'local>, placeholder_name: JString<'local>, placeholder_args: JString<'local>, leadingIcon_name: JString<'local>, leadingIcon_args: JString<'local>, trailingIcon_name: JString<'local>, trailingIcon_args: JString<'local>, supportingText_name: JString<'local>, supportingText_args: JString<'local>, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
	let jvm = get_jvm(env);
	let TextField_composable = if TextField_args.is_null() {
		Composable::new(env.get_string(TextField_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(TextField_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &TextField_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ContainerBox_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape) -> jobject {
	let jvm = get_jvm(env);
	let ContainerBox_composable = if ContainerBox_args.is_null() {
		Composable::new(env.get_string(ContainerBox_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(ContainerBox_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &ContainerBox_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, focusedTextColor: Color, unfocusedTextColor: Color, disabledTextColor: Color, errorTextColor: Color, focusedContainerColor: Color, unfocusedContainerColor: Color, disabledContainerColor: Color, errorContainerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedIndicatorColor: Color, unfocusedIndicatorColor: Color, disabledIndicatorColor: Color, errorIndicatorColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, focusedPlaceholderColor: Color, unfocusedPlaceholderColor: Color, disabledPlaceholderColor: Color, errorPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_FilledContainerBox_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape) -> jobject {
	let jvm = get_jvm(env);
	let FilledContainerBox_composable = if FilledContainerBox_args.is_null() {
		Composable::new(env.get_string(FilledContainerBox_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(FilledContainerBox_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &FilledContainerBox_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedBorderContainerBox_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape, focusedBorderThickness: Dp, unfocusedBorderThickness: Dp) -> jobject {
	let jvm = get_jvm(env);
	let OutlinedBorderContainerBox_composable = if OutlinedBorderContainerBox_args.is_null() {
		Composable::new(env.get_string(OutlinedBorderContainerBox_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(OutlinedBorderContainerBox_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &OutlinedBorderContainerBox_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ContainerBox_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape, focusedBorderThickness: Dp, unfocusedBorderThickness: Dp) -> jobject {
	let jvm = get_jvm(env);
	let ContainerBox_composable = if ContainerBox_args.is_null() {
		Composable::new(env.get_string(ContainerBox_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(ContainerBox_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &ContainerBox_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, focusedTextColor: Color, unfocusedTextColor: Color, disabledTextColor: Color, errorTextColor: Color, focusedContainerColor: Color, unfocusedContainerColor: Color, disabledContainerColor: Color, errorContainerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedBorderColor: Color, unfocusedBorderColor: Color, disabledBorderColor: Color, errorBorderColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, focusedPlaceholderColor: Color, unfocusedPlaceholderColor: Color, disabledPlaceholderColor: Color, errorPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Tab_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean, onClick: () -> Unit, modifier: Modifier, enabled: Boolean, text_name: JString<'local>, text_args: JString<'local>, icon_name: JString<'local>, icon_args: JString<'local>, selectedContentColor: Color, unselectedContentColor: Color, interactionSource: MutableInteractionSource) -> jobject {
	let jvm = get_jvm(env);
	let Tab_composable = if Tab_args.is_null() {
		Composable::new(env.get_string(Tab_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Tab_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Tab_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_LeadingIconTab_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean, onClick: () -> Unit, text_name: JString<'local>, text_args: JString<'local>, icon_name: JString<'local>, icon_args: JString<'local>, modifier: Modifier, enabled: Boolean, selectedContentColor: Color, unselectedContentColor: Color, interactionSource: MutableInteractionSource) -> jobject {
	let jvm = get_jvm(env);
	let LeadingIconTab_composable = if LeadingIconTab_args.is_null() {
		Composable::new(env.get_string(LeadingIconTab_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(LeadingIconTab_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &LeadingIconTab_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Tab_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean, onClick: () -> Unit, modifier: Modifier, enabled: Boolean, selectedContentColor: Color, unselectedContentColor: Color, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let Tab_composable = if Tab_args.is_null() {
		Composable::new(env.get_string(Tab_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Tab_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Tab_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_FloatingActionButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let FloatingActionButton_composable = if FloatingActionButton_args.is_null() {
		Composable::new(env.get_string(FloatingActionButton_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(FloatingActionButton_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &FloatingActionButton_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_SmallFloatingActionButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let SmallFloatingActionButton_composable = if SmallFloatingActionButton_args.is_null() {
		Composable::new(env.get_string(SmallFloatingActionButton_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(SmallFloatingActionButton_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &SmallFloatingActionButton_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_LargeFloatingActionButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let LargeFloatingActionButton_composable = if LargeFloatingActionButton_args.is_null() {
		Composable::new(env.get_string(LargeFloatingActionButton_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(LargeFloatingActionButton_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &LargeFloatingActionButton_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ExtendedFloatingActionButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let ExtendedFloatingActionButton_composable = if ExtendedFloatingActionButton_args.is_null() {
		Composable::new(env.get_string(ExtendedFloatingActionButton_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(ExtendedFloatingActionButton_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &ExtendedFloatingActionButton_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ExtendedFloatingActionButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, text_name: JString<'local>, text_args: JString<'local>, icon_name: JString<'local>, icon_args: JString<'local>, onClick: () -> Unit, modifier: Modifier, expanded: Boolean, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource) -> jobject {
	let jvm = get_jvm(env);
	let ExtendedFloatingActionButton_composable = if ExtendedFloatingActionButton_args.is_null() {
		Composable::new(env.get_string(ExtendedFloatingActionButton_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(ExtendedFloatingActionButton_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &ExtendedFloatingActionButton_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp) -> jobject {
	let jvm = get_jvm(env);
	let elevation_composable = if elevation_args.is_null() {
		Composable::new(env.get_string(elevation_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(elevation_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &elevation_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_loweredElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp) -> jobject {
	let jvm = get_jvm(env);
	let loweredElevation_composable = if loweredElevation_args.is_null() {
		Composable::new(env.get_string(loweredElevation_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(loweredElevation_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &loweredElevation_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_IconButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, modifier: Modifier, enabled: Boolean, colors: IconButtonColors, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let IconButton_composable = if IconButton_args.is_null() {
		Composable::new(env.get_string(IconButton_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(IconButton_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &IconButton_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_IconToggleButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, checked: Boolean, onCheckedChange: (Boolean) -> Unit, modifier: Modifier, enabled: Boolean, colors: IconToggleButtonColors, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let IconToggleButton_composable = if IconToggleButton_args.is_null() {
		Composable::new(env.get_string(IconToggleButton_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(IconToggleButton_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &IconToggleButton_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_FilledIconButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconButtonColors, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let FilledIconButton_composable = if FilledIconButton_args.is_null() {
		Composable::new(env.get_string(FilledIconButton_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(FilledIconButton_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &FilledIconButton_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_FilledTonalIconButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconButtonColors, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let FilledTonalIconButton_composable = if FilledTonalIconButton_args.is_null() {
		Composable::new(env.get_string(FilledTonalIconButton_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(FilledTonalIconButton_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &FilledTonalIconButton_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_FilledIconToggleButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, checked: Boolean, onCheckedChange: (Boolean) -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconToggleButtonColors, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let FilledIconToggleButton_composable = if FilledIconToggleButton_args.is_null() {
		Composable::new(env.get_string(FilledIconToggleButton_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(FilledIconToggleButton_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &FilledIconToggleButton_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_FilledTonalIconToggleButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, checked: Boolean, onCheckedChange: (Boolean) -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconToggleButtonColors, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let FilledTonalIconToggleButton_composable = if FilledTonalIconToggleButton_args.is_null() {
		Composable::new(env.get_string(FilledTonalIconToggleButton_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(FilledTonalIconToggleButton_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &FilledTonalIconToggleButton_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedIconButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconButtonColors, border: BorderStroke, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let OutlinedIconButton_composable = if OutlinedIconButton_args.is_null() {
		Composable::new(env.get_string(OutlinedIconButton_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(OutlinedIconButton_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &OutlinedIconButton_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedIconToggleButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, checked: Boolean, onCheckedChange: (Boolean) -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconToggleButtonColors, border: BorderStroke, interactionSource: MutableInteractionSource, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let OutlinedIconToggleButton_composable = if OutlinedIconToggleButton_args.is_null() {
		Composable::new(env.get_string(OutlinedIconToggleButton_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(OutlinedIconToggleButton_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &OutlinedIconToggleButton_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_iconButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let iconButtonColors_composable = if iconButtonColors_args.is_null() {
		Composable::new(env.get_string(iconButtonColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(iconButtonColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &iconButtonColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_filledIconButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let filledIconButtonColors_composable = if filledIconButtonColors_args.is_null() {
		Composable::new(env.get_string(filledIconButtonColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(filledIconButtonColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &filledIconButtonColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_filledTonalIconButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let filledTonalIconButtonColors_composable = if filledTonalIconButtonColors_args.is_null() {
		Composable::new(env.get_string(filledTonalIconButtonColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(filledTonalIconButtonColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &filledTonalIconButtonColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_filledTonalIconToggleButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let filledTonalIconToggleButtonColors_composable = if filledTonalIconToggleButtonColors_args.is_null() {
		Composable::new(env.get_string(filledTonalIconToggleButtonColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(filledTonalIconToggleButtonColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &filledTonalIconToggleButtonColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_outlinedIconToggleButtonBorder_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, enabled: Boolean, checked: Boolean) -> jobject {
	let jvm = get_jvm(env);
	let outlinedIconToggleButtonBorder_composable = if outlinedIconToggleButtonBorder_args.is_null() {
		Composable::new(env.get_string(outlinedIconToggleButtonBorder_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(outlinedIconToggleButtonBorder_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &outlinedIconToggleButtonBorder_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_outlinedIconButtonBorder_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, enabled: Boolean) -> jobject {
	let jvm = get_jvm(env);
	let outlinedIconButtonBorder_composable = if outlinedIconButtonBorder_args.is_null() {
		Composable::new(env.get_string(outlinedIconButtonBorder_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(outlinedIconButtonBorder_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &outlinedIconButtonBorder_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_NavigationRail_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, containerColor: Color, contentColor: Color, header_name: JString<'local>, header_args: JString<'local>, windowInsets: WindowInsets, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let NavigationRail_composable = if NavigationRail_args.is_null() {
		Composable::new(env.get_string(NavigationRail_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(NavigationRail_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &NavigationRail_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_NavigationRailItem_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean, onClick: () -> Unit, icon_name: JString<'local>, icon_args: JString<'local>, modifier: Modifier, enabled: Boolean, label_name: JString<'local>, label_args: JString<'local>, alwaysShowLabel: Boolean, colors: NavigationRailItemColors, interactionSource: MutableInteractionSource) -> jobject {
	let jvm = get_jvm(env);
	let NavigationRailItem_composable = if NavigationRailItem_args.is_null() {
		Composable::new(env.get_string(NavigationRailItem_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(NavigationRailItem_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &NavigationRailItem_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, disabledIconColor: Color, disabledTextColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_rememberDrawerState_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, initialValue: DrawerValue, confirmStateChange: (DrawerValue) -> Boolean) -> jobject {
	let jvm = get_jvm(env);
	let rememberDrawerState_composable = if rememberDrawerState_args.is_null() {
		Composable::new(env.get_string(rememberDrawerState_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(rememberDrawerState_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &rememberDrawerState_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ModalNavigationDrawer_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, drawerContent_name: JString<'local>, drawerContent_args: JString<'local>, modifier: Modifier, drawerState: DrawerState, gesturesEnabled: Boolean, scrimColor: Color, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let ModalNavigationDrawer_composable = if ModalNavigationDrawer_args.is_null() {
		Composable::new(env.get_string(ModalNavigationDrawer_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(ModalNavigationDrawer_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &ModalNavigationDrawer_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_DismissibleNavigationDrawer_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, drawerContent_name: JString<'local>, drawerContent_args: JString<'local>, modifier: Modifier, drawerState: DrawerState, gesturesEnabled: Boolean, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let DismissibleNavigationDrawer_composable = if DismissibleNavigationDrawer_args.is_null() {
		Composable::new(env.get_string(DismissibleNavigationDrawer_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(DismissibleNavigationDrawer_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &DismissibleNavigationDrawer_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_PermanentNavigationDrawer_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, drawerContent_name: JString<'local>, drawerContent_args: JString<'local>, modifier: Modifier, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let PermanentNavigationDrawer_composable = if PermanentNavigationDrawer_args.is_null() {
		Composable::new(env.get_string(PermanentNavigationDrawer_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(PermanentNavigationDrawer_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &PermanentNavigationDrawer_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ModalDrawerSheet_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, drawerShape: Shape, drawerContainerColor: Color, drawerContentColor: Color, drawerTonalElevation: Dp, windowInsets: WindowInsets, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let ModalDrawerSheet_composable = if ModalDrawerSheet_args.is_null() {
		Composable::new(env.get_string(ModalDrawerSheet_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(ModalDrawerSheet_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &ModalDrawerSheet_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_DismissibleDrawerSheet_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, drawerShape: Shape, drawerContainerColor: Color, drawerContentColor: Color, drawerTonalElevation: Dp, windowInsets: WindowInsets, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let DismissibleDrawerSheet_composable = if DismissibleDrawerSheet_args.is_null() {
		Composable::new(env.get_string(DismissibleDrawerSheet_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(DismissibleDrawerSheet_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &DismissibleDrawerSheet_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_PermanentDrawerSheet_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, drawerShape: Shape, drawerContainerColor: Color, drawerContentColor: Color, drawerTonalElevation: Dp, windowInsets: WindowInsets, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let PermanentDrawerSheet_composable = if PermanentDrawerSheet_args.is_null() {
		Composable::new(env.get_string(PermanentDrawerSheet_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(PermanentDrawerSheet_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &PermanentDrawerSheet_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_NavigationDrawerItem_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, label_name: JString<'local>, label_args: JString<'local>, selected: Boolean, onClick: () -> Unit, modifier: Modifier, icon: (@Composable () -> Unit), badge: (@Composable () -> Unit), shape: Shape, colors: NavigationDrawerItemColors, interactionSource: MutableInteractionSource) -> jobject {
	let jvm = get_jvm(env);
	let NavigationDrawerItem_composable = if NavigationDrawerItem_args.is_null() {
		Composable::new(env.get_string(NavigationDrawerItem_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(NavigationDrawerItem_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &NavigationDrawerItem_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_iconColor_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean) -> jobject {
	let jvm = get_jvm(env);
	let iconColor_composable = if iconColor_args.is_null() {
		Composable::new(env.get_string(iconColor_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(iconColor_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &iconColor_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_textColor_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean) -> jobject {
	let jvm = get_jvm(env);
	let textColor_composable = if textColor_args.is_null() {
		Composable::new(env.get_string(textColor_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(textColor_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &textColor_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_badgeColor_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean) -> jobject {
	let jvm = get_jvm(env);
	let badgeColor_composable = if badgeColor_args.is_null() {
		Composable::new(env.get_string(badgeColor_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(badgeColor_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &badgeColor_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_containerColor_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean) -> jobject {
	let jvm = get_jvm(env);
	let containerColor_composable = if containerColor_args.is_null() {
		Composable::new(env.get_string(containerColor_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(containerColor_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &containerColor_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedContainerColor: Color, unselectedContainerColor: Color, selectedIconColor: Color, unselectedIconColor: Color, selectedTextColor: Color, unselectedTextColor: Color, selectedBadgeColor: Color, unselectedBadgeColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_PrimaryTabRow_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, indicator_name: JString<'local>, indicator_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let PrimaryTabRow_composable = if PrimaryTabRow_args.is_null() {
		Composable::new(env.get_string(PrimaryTabRow_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(PrimaryTabRow_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &PrimaryTabRow_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_SecondaryTabRow_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, indicator_name: JString<'local>, indicator_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let SecondaryTabRow_composable = if SecondaryTabRow_args.is_null() {
		Composable::new(env.get_string(SecondaryTabRow_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(SecondaryTabRow_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &SecondaryTabRow_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_TabRow_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, indicator_name: JString<'local>, indicator_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let TabRow_composable = if TabRow_args.is_null() {
		Composable::new(env.get_string(TabRow_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(TabRow_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &TabRow_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_PrimaryScrollableTabRow_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedTabIndex: Int, modifier: Modifier, scrollState: ScrollState, containerColor: Color, contentColor: Color, edgePadding: Dp, indicator_name: JString<'local>, indicator_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let PrimaryScrollableTabRow_composable = if PrimaryScrollableTabRow_args.is_null() {
		Composable::new(env.get_string(PrimaryScrollableTabRow_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(PrimaryScrollableTabRow_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &PrimaryScrollableTabRow_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_SecondaryScrollableTabRow_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedTabIndex: Int, modifier: Modifier, scrollState: ScrollState, containerColor: Color, contentColor: Color, edgePadding: Dp, indicator_name: JString<'local>, indicator_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let SecondaryScrollableTabRow_composable = if SecondaryScrollableTabRow_args.is_null() {
		Composable::new(env.get_string(SecondaryScrollableTabRow_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(SecondaryScrollableTabRow_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &SecondaryScrollableTabRow_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ScrollableTabRow_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, edgePadding: Dp, indicator_name: JString<'local>, indicator_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let ScrollableTabRow_composable = if ScrollableTabRow_args.is_null() {
		Composable::new(env.get_string(ScrollableTabRow_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(ScrollableTabRow_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &ScrollableTabRow_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_PrimaryIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, width: Dp, height: Dp, color: Color, shape: Shape) -> jobject {
	let jvm = get_jvm(env);
	let PrimaryIndicator_composable = if PrimaryIndicator_args.is_null() {
		Composable::new(env.get_string(PrimaryIndicator_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(PrimaryIndicator_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &PrimaryIndicator_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_SecondaryIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, height: Dp, color: Color) -> jobject {
	let jvm = get_jvm(env);
	let SecondaryIndicator_composable = if SecondaryIndicator_args.is_null() {
		Composable::new(env.get_string(SecondaryIndicator_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(SecondaryIndicator_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &SecondaryIndicator_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, activeContainerColor: Color, activeContentColor: Color, activeBorderColor: Color, inactiveContainerColor: Color, inactiveContentColor: Color, inactiveBorderColor: Color, disabledActiveContainerColor: Color, disabledActiveContentColor: Color, disabledActiveBorderColor: Color, disabledInactiveContainerColor: Color, disabledInactiveContentColor: Color, disabledInactiveBorderColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ActiveIcon_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let ActiveIcon_composable = if ActiveIcon_args.is_null() {
		Composable::new(env.get_string(ActiveIcon_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(ActiveIcon_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &ActiveIcon_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Icon_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, active: Boolean, activeContent_name: JString<'local>, activeContent_args: JString<'local>, inactiveContent: (@Composable () -> Unit)) -> jobject {
	let jvm = get_jvm(env);
	let Icon_composable = if Icon_args.is_null() {
		Composable::new(env.get_string(Icon_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Icon_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Icon_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Icon_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, imageVector: ImageVector, contentDescription: String?, modifier: Modifier, tint: Color) -> jobject {
	let jvm = get_jvm(env);
	let Icon_composable = if Icon_args.is_null() {
		Composable::new(env.get_string(Icon_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Icon_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Icon_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Icon_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, bitmap: ImageBitmap, contentDescription: String?, modifier: Modifier, tint: Color) -> jobject {
	let jvm = get_jvm(env);
	let Icon_composable = if Icon_args.is_null() {
		Composable::new(env.get_string(Icon_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Icon_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Icon_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Icon_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, painter: Painter, contentDescription: String?, modifier: Modifier, tint: Color) -> jobject {
	let jvm = get_jvm(env);
	let Icon_composable = if Icon_args.is_null() {
		Composable::new(env.get_string(Icon_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Icon_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Icon_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_TopAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, title_name: JString<'local>, title_args: JString<'local>, modifier: Modifier, navigationIcon_name: JString<'local>, navigationIcon_args: JString<'local>, actions_name: JString<'local>, actions_args: JString<'local>, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior) -> jobject {
	let jvm = get_jvm(env);
	let TopAppBar_composable = if TopAppBar_args.is_null() {
		Composable::new(env.get_string(TopAppBar_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(TopAppBar_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &TopAppBar_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_SmallTopAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, title_name: JString<'local>, title_args: JString<'local>, modifier: Modifier, navigationIcon_name: JString<'local>, navigationIcon_args: JString<'local>, actions_name: JString<'local>, actions_args: JString<'local>, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior) -> jobject {
	let jvm = get_jvm(env);
	let SmallTopAppBar_composable = if SmallTopAppBar_args.is_null() {
		Composable::new(env.get_string(SmallTopAppBar_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(SmallTopAppBar_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &SmallTopAppBar_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_CenterAlignedTopAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, title_name: JString<'local>, title_args: JString<'local>, modifier: Modifier, navigationIcon_name: JString<'local>, navigationIcon_args: JString<'local>, actions_name: JString<'local>, actions_args: JString<'local>, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior) -> jobject {
	let jvm = get_jvm(env);
	let CenterAlignedTopAppBar_composable = if CenterAlignedTopAppBar_args.is_null() {
		Composable::new(env.get_string(CenterAlignedTopAppBar_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(CenterAlignedTopAppBar_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &CenterAlignedTopAppBar_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_MediumTopAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, title_name: JString<'local>, title_args: JString<'local>, modifier: Modifier, navigationIcon_name: JString<'local>, navigationIcon_args: JString<'local>, actions_name: JString<'local>, actions_args: JString<'local>, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior) -> jobject {
	let jvm = get_jvm(env);
	let MediumTopAppBar_composable = if MediumTopAppBar_args.is_null() {
		Composable::new(env.get_string(MediumTopAppBar_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(MediumTopAppBar_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &MediumTopAppBar_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_LargeTopAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, title_name: JString<'local>, title_args: JString<'local>, modifier: Modifier, navigationIcon_name: JString<'local>, navigationIcon_args: JString<'local>, actions_name: JString<'local>, actions_args: JString<'local>, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior) -> jobject {
	let jvm = get_jvm(env);
	let LargeTopAppBar_composable = if LargeTopAppBar_args.is_null() {
		Composable::new(env.get_string(LargeTopAppBar_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(LargeTopAppBar_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &LargeTopAppBar_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_BottomAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, actions_name: JString<'local>, actions_args: JString<'local>, modifier: Modifier, floatingActionButton_name: JString<'local>, floatingActionButton_args: JString<'local>, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets) -> jobject {
	let jvm = get_jvm(env);
	let BottomAppBar_composable = if BottomAppBar_args.is_null() {
		Composable::new(env.get_string(BottomAppBar_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(BottomAppBar_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &BottomAppBar_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_BottomAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, actions_name: JString<'local>, actions_args: JString<'local>, modifier: Modifier, floatingActionButton_name: JString<'local>, floatingActionButton_args: JString<'local>, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, scrollBehavior: BottomAppBarScrollBehavior) -> jobject {
	let jvm = get_jvm(env);
	let BottomAppBar_composable = if BottomAppBar_args.is_null() {
		Composable::new(env.get_string(BottomAppBar_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(BottomAppBar_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &BottomAppBar_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_BottomAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let BottomAppBar_composable = if BottomAppBar_args.is_null() {
		Composable::new(env.get_string(BottomAppBar_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(BottomAppBar_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &BottomAppBar_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_BottomAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, scrollBehavior: BottomAppBarScrollBehavior, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let BottomAppBar_composable = if BottomAppBar_args.is_null() {
		Composable::new(env.get_string(BottomAppBar_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(BottomAppBar_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &BottomAppBar_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_topAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let topAppBarColors_composable = if topAppBarColors_args.is_null() {
		Composable::new(env.get_string(topAppBarColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(topAppBarColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &topAppBarColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_topAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let topAppBarColors_composable = if topAppBarColors_args.is_null() {
		Composable::new(env.get_string(topAppBarColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(topAppBarColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &topAppBarColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_centerAlignedTopAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let centerAlignedTopAppBarColors_composable = if centerAlignedTopAppBarColors_args.is_null() {
		Composable::new(env.get_string(centerAlignedTopAppBarColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(centerAlignedTopAppBarColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &centerAlignedTopAppBarColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_centerAlignedTopAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let centerAlignedTopAppBarColors_composable = if centerAlignedTopAppBarColors_args.is_null() {
		Composable::new(env.get_string(centerAlignedTopAppBarColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(centerAlignedTopAppBarColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &centerAlignedTopAppBarColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_mediumTopAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let mediumTopAppBarColors_composable = if mediumTopAppBarColors_args.is_null() {
		Composable::new(env.get_string(mediumTopAppBarColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(mediumTopAppBarColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &mediumTopAppBarColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_mediumTopAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let mediumTopAppBarColors_composable = if mediumTopAppBarColors_args.is_null() {
		Composable::new(env.get_string(mediumTopAppBarColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(mediumTopAppBarColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &mediumTopAppBarColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_largeTopAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let largeTopAppBarColors_composable = if largeTopAppBarColors_args.is_null() {
		Composable::new(env.get_string(largeTopAppBarColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(largeTopAppBarColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &largeTopAppBarColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_largeTopAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let largeTopAppBarColors_composable = if largeTopAppBarColors_args.is_null() {
		Composable::new(env.get_string(largeTopAppBarColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(largeTopAppBarColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &largeTopAppBarColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_pinnedScrollBehavior_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: TopAppBarState, canScroll: () -> Boolean) -> jobject {
	let jvm = get_jvm(env);
	let pinnedScrollBehavior_composable = if pinnedScrollBehavior_args.is_null() {
		Composable::new(env.get_string(pinnedScrollBehavior_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(pinnedScrollBehavior_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &pinnedScrollBehavior_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_enterAlwaysScrollBehavior_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: TopAppBarState, canScroll: () -> Boolean, snapAnimationSpec: AnimationSpec<Float>, flingAnimationSpec: DecayAnimationSpec<Float>) -> jobject {
	let jvm = get_jvm(env);
	let enterAlwaysScrollBehavior_composable = if enterAlwaysScrollBehavior_args.is_null() {
		Composable::new(env.get_string(enterAlwaysScrollBehavior_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(enterAlwaysScrollBehavior_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &enterAlwaysScrollBehavior_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_exitUntilCollapsedScrollBehavior_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: TopAppBarState, canScroll: () -> Boolean, snapAnimationSpec: AnimationSpec<Float>, flingAnimationSpec: DecayAnimationSpec<Float>) -> jobject {
	let jvm = get_jvm(env);
	let exitUntilCollapsedScrollBehavior_composable = if exitUntilCollapsedScrollBehavior_args.is_null() {
		Composable::new(env.get_string(exitUntilCollapsedScrollBehavior_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(exitUntilCollapsedScrollBehavior_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &exitUntilCollapsedScrollBehavior_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_rememberTopAppBarState_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, initialHeightOffsetLimit: Float, initialHeightOffset: Float, initialContentOffset: Float) -> jobject {
	let jvm = get_jvm(env);
	let rememberTopAppBarState_composable = if rememberTopAppBarState_args.is_null() {
		Composable::new(env.get_string(rememberTopAppBarState_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(rememberTopAppBarState_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &rememberTopAppBarState_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_exitAlwaysScrollBehavior_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: BottomAppBarState, canScroll: () -> Boolean, snapAnimationSpec: AnimationSpec<Float>, flingAnimationSpec: DecayAnimationSpec<Float>) -> jobject {
	let jvm = get_jvm(env);
	let exitAlwaysScrollBehavior_composable = if exitAlwaysScrollBehavior_args.is_null() {
		Composable::new(env.get_string(exitAlwaysScrollBehavior_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(exitAlwaysScrollBehavior_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &exitAlwaysScrollBehavior_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_rememberBottomAppBarState_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, initialHeightOffsetLimit: Float, initialHeightOffset: Float, initialContentOffset: Float) -> jobject {
	let jvm = get_jvm(env);
	let rememberBottomAppBarState_composable = if rememberBottomAppBarState_args.is_null() {
		Composable::new(env.get_string(rememberBottomAppBarState_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(rememberBottomAppBarState_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &rememberBottomAppBarState_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedTextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: String, onValueChange: (String) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label_name: JString<'local>, label_args: JString<'local>, placeholder_name: JString<'local>, placeholder_args: JString<'local>, leadingIcon_name: JString<'local>, leadingIcon_args: JString<'local>, trailingIcon_name: JString<'local>, trailingIcon_args: JString<'local>, prefix_name: JString<'local>, prefix_args: JString<'local>, suffix_name: JString<'local>, suffix_args: JString<'local>, supportingText_name: JString<'local>, supportingText_args: JString<'local>, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
	let jvm = get_jvm(env);
	let OutlinedTextField_composable = if OutlinedTextField_args.is_null() {
		Composable::new(env.get_string(OutlinedTextField_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(OutlinedTextField_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &OutlinedTextField_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedTextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: TextFieldValue, onValueChange: (TextFieldValue) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label_name: JString<'local>, label_args: JString<'local>, placeholder_name: JString<'local>, placeholder_args: JString<'local>, leadingIcon_name: JString<'local>, leadingIcon_args: JString<'local>, trailingIcon_name: JString<'local>, trailingIcon_args: JString<'local>, prefix_name: JString<'local>, prefix_args: JString<'local>, suffix_name: JString<'local>, suffix_args: JString<'local>, supportingText_name: JString<'local>, supportingText_args: JString<'local>, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
	let jvm = get_jvm(env);
	let OutlinedTextField_composable = if OutlinedTextField_args.is_null() {
		Composable::new(env.get_string(OutlinedTextField_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(OutlinedTextField_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &OutlinedTextField_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedTextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: String, onValueChange: (String) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label_name: JString<'local>, label_args: JString<'local>, placeholder_name: JString<'local>, placeholder_args: JString<'local>, leadingIcon_name: JString<'local>, leadingIcon_args: JString<'local>, trailingIcon_name: JString<'local>, trailingIcon_args: JString<'local>, supportingText_name: JString<'local>, supportingText_args: JString<'local>, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
	let jvm = get_jvm(env);
	let OutlinedTextField_composable = if OutlinedTextField_args.is_null() {
		Composable::new(env.get_string(OutlinedTextField_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(OutlinedTextField_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &OutlinedTextField_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedTextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: TextFieldValue, onValueChange: (TextFieldValue) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label_name: JString<'local>, label_args: JString<'local>, placeholder_name: JString<'local>, placeholder_args: JString<'local>, leadingIcon_name: JString<'local>, leadingIcon_args: JString<'local>, trailingIcon_name: JString<'local>, trailingIcon_args: JString<'local>, supportingText_name: JString<'local>, supportingText_args: JString<'local>, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
	let jvm = get_jvm(env);
	let OutlinedTextField_composable = if OutlinedTextField_args.is_null() {
		Composable::new(env.get_string(OutlinedTextField_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(OutlinedTextField_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &OutlinedTextField_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Snackbar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, action_name: JString<'local>, action_args: JString<'local>, dismissAction_name: JString<'local>, dismissAction_args: JString<'local>, actionOnNewLine: Boolean, shape: Shape, containerColor: Color, contentColor: Color, actionContentColor: Color, dismissActionContentColor: Color, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let Snackbar_composable = if Snackbar_args.is_null() {
		Composable::new(env.get_string(Snackbar_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Snackbar_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Snackbar_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Snackbar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, snackbarData: SnackbarData, modifier: Modifier, actionOnNewLine: Boolean, shape: Shape, containerColor: Color, contentColor: Color, actionColor: Color, actionContentColor: Color, dismissActionContentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let Snackbar_composable = if Snackbar_args.is_null() {
		Composable::new(env.get_string(Snackbar_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Snackbar_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Snackbar_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ListItem_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, headlineContent_name: JString<'local>, headlineContent_args: JString<'local>, modifier: Modifier, overlineContent_name: JString<'local>, overlineContent_args: JString<'local>, supportingContent_name: JString<'local>, supportingContent_args: JString<'local>, leadingContent_name: JString<'local>, leadingContent_args: JString<'local>, trailingContent_name: JString<'local>, trailingContent_args: JString<'local>, colors: ListItemColors, tonalElevation: Dp, shadowElevation: Dp) -> jobject {
	let jvm = get_jvm(env);
	let ListItem_composable = if ListItem_args.is_null() {
		Composable::new(env.get_string(ListItem_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(ListItem_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &ListItem_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_RadioButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean, onClick: KotlinCallback::new(vec![], "Unit"), modifier: Modifier, enabled: Boolean, colors: RadioButtonColors, interactionSource: MutableInteractionSource) -> jobject {
	let jvm = get_jvm(env);
	let RadioButton_composable = if RadioButton_args.is_null() {
		Composable::new(env.get_string(RadioButton_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(RadioButton_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &RadioButton_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedColor: Color, unselectedColor: Color, disabledSelectedColor: Color, disabledUnselectedColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let colors_composable = if colors_args.is_null() {
		Composable::new(env.get_string(colors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(colors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &colors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_BadgedBox_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, badge_name: JString<'local>, badge_args: JString<'local>, modifier: Modifier, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let BadgedBox_composable = if BadgedBox_args.is_null() {
		Composable::new(env.get_string(BadgedBox_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(BadgedBox_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &BadgedBox_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Badge_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, containerColor: Color, contentColor: Color, content_name: JString<'local>, content_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let Badge_composable = if Badge_args.is_null() {
		Composable::new(env.get_string(Badge_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Badge_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Badge_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_richTooltipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let richTooltipColors_composable = if richTooltipColors_args.is_null() {
		Composable::new(env.get_string(richTooltipColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(richTooltipColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &richTooltipColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_richTooltipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, titleContentColor: Color, actionContentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let richTooltipColors_composable = if richTooltipColors_args.is_null() {
		Composable::new(env.get_string(richTooltipColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(richTooltipColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &richTooltipColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_rememberPlainTooltipPositionProvider_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, spacingBetweenTooltipAndAnchor: Dp) -> jobject {
	let jvm = get_jvm(env);
	let rememberPlainTooltipPositionProvider_composable = if rememberPlainTooltipPositionProvider_args.is_null() {
		Composable::new(env.get_string(rememberPlainTooltipPositionProvider_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(rememberPlainTooltipPositionProvider_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &rememberPlainTooltipPositionProvider_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_rememberRichTooltipPositionProvider_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, spacingBetweenTooltipAndAnchor: Dp) -> jobject {
	let jvm = get_jvm(env);
	let rememberRichTooltipPositionProvider_composable = if rememberRichTooltipPositionProvider_args.is_null() {
		Composable::new(env.get_string(rememberRichTooltipPositionProvider_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(rememberRichTooltipPositionProvider_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &rememberRichTooltipPositionProvider_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_AssistChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, label_name: JString<'local>, label_args: JString<'local>, modifier: Modifier, enabled: Boolean, leadingIcon_name: JString<'local>, leadingIcon_args: JString<'local>, trailingIcon_name: JString<'local>, trailingIcon_args: JString<'local>, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource) -> jobject {
	let jvm = get_jvm(env);
	let AssistChip_composable = if AssistChip_args.is_null() {
		Composable::new(env.get_string(AssistChip_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(AssistChip_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &AssistChip_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_AssistChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, label_name: JString<'local>, label_args: JString<'local>, modifier: Modifier, enabled: Boolean, leadingIcon_name: JString<'local>, leadingIcon_args: JString<'local>, trailingIcon_name: JString<'local>, trailingIcon_args: JString<'local>, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource) -> jobject {
	let jvm = get_jvm(env);
	let AssistChip_composable = if AssistChip_args.is_null() {
		Composable::new(env.get_string(AssistChip_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(AssistChip_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &AssistChip_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ElevatedAssistChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, label_name: JString<'local>, label_args: JString<'local>, modifier: Modifier, enabled: Boolean, leadingIcon_name: JString<'local>, leadingIcon_args: JString<'local>, trailingIcon_name: JString<'local>, trailingIcon_args: JString<'local>, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource) -> jobject {
	let jvm = get_jvm(env);
	let ElevatedAssistChip_composable = if ElevatedAssistChip_args.is_null() {
		Composable::new(env.get_string(ElevatedAssistChip_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(ElevatedAssistChip_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &ElevatedAssistChip_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ElevatedAssistChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, label_name: JString<'local>, label_args: JString<'local>, modifier: Modifier, enabled: Boolean, leadingIcon_name: JString<'local>, leadingIcon_args: JString<'local>, trailingIcon_name: JString<'local>, trailingIcon_args: JString<'local>, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource) -> jobject {
	let jvm = get_jvm(env);
	let ElevatedAssistChip_composable = if ElevatedAssistChip_args.is_null() {
		Composable::new(env.get_string(ElevatedAssistChip_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(ElevatedAssistChip_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &ElevatedAssistChip_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_FilterChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean, onClick: () -> Unit, label_name: JString<'local>, label_args: JString<'local>, modifier: Modifier, enabled: Boolean, leadingIcon_name: JString<'local>, leadingIcon_args: JString<'local>, trailingIcon_name: JString<'local>, trailingIcon_args: JString<'local>, shape: Shape, colors: SelectableChipColors, elevation: SelectableChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource) -> jobject {
	let jvm = get_jvm(env);
	let FilterChip_composable = if FilterChip_args.is_null() {
		Composable::new(env.get_string(FilterChip_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(FilterChip_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &FilterChip_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ElevatedFilterChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean, onClick: () -> Unit, label_name: JString<'local>, label_args: JString<'local>, modifier: Modifier, enabled: Boolean, leadingIcon_name: JString<'local>, leadingIcon_args: JString<'local>, trailingIcon_name: JString<'local>, trailingIcon_args: JString<'local>, shape: Shape, colors: SelectableChipColors, elevation: SelectableChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource) -> jobject {
	let jvm = get_jvm(env);
	let ElevatedFilterChip_composable = if ElevatedFilterChip_args.is_null() {
		Composable::new(env.get_string(ElevatedFilterChip_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(ElevatedFilterChip_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &ElevatedFilterChip_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_InputChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean, onClick: () -> Unit, label_name: JString<'local>, label_args: JString<'local>, modifier: Modifier, enabled: Boolean, leadingIcon_name: JString<'local>, leadingIcon_args: JString<'local>, avatar_name: JString<'local>, avatar_args: JString<'local>, trailingIcon_name: JString<'local>, trailingIcon_args: JString<'local>, shape: Shape, colors: SelectableChipColors, elevation: SelectableChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource) -> jobject {
	let jvm = get_jvm(env);
	let InputChip_composable = if InputChip_args.is_null() {
		Composable::new(env.get_string(InputChip_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(InputChip_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &InputChip_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_SuggestionChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, label_name: JString<'local>, label_args: JString<'local>, modifier: Modifier, enabled: Boolean, icon_name: JString<'local>, icon_args: JString<'local>, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource) -> jobject {
	let jvm = get_jvm(env);
	let SuggestionChip_composable = if SuggestionChip_args.is_null() {
		Composable::new(env.get_string(SuggestionChip_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(SuggestionChip_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &SuggestionChip_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_SuggestionChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, label_name: JString<'local>, label_args: JString<'local>, modifier: Modifier, enabled: Boolean, icon_name: JString<'local>, icon_args: JString<'local>, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource) -> jobject {
	let jvm = get_jvm(env);
	let SuggestionChip_composable = if SuggestionChip_args.is_null() {
		Composable::new(env.get_string(SuggestionChip_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(SuggestionChip_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &SuggestionChip_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ElevatedSuggestionChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, label_name: JString<'local>, label_args: JString<'local>, modifier: Modifier, enabled: Boolean, icon_name: JString<'local>, icon_args: JString<'local>, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource) -> jobject {
	let jvm = get_jvm(env);
	let ElevatedSuggestionChip_composable = if ElevatedSuggestionChip_args.is_null() {
		Composable::new(env.get_string(ElevatedSuggestionChip_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(ElevatedSuggestionChip_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &ElevatedSuggestionChip_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ElevatedSuggestionChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: () -> Unit, label_name: JString<'local>, label_args: JString<'local>, modifier: Modifier, enabled: Boolean, icon_name: JString<'local>, icon_args: JString<'local>, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource) -> jobject {
	let jvm = get_jvm(env);
	let ElevatedSuggestionChip_composable = if ElevatedSuggestionChip_args.is_null() {
		Composable::new(env.get_string(ElevatedSuggestionChip_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(ElevatedSuggestionChip_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &ElevatedSuggestionChip_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_assistChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let assistChipColors_composable = if assistChipColors_args.is_null() {
		Composable::new(env.get_string(assistChipColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(assistChipColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &assistChipColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_assistChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, labelColor: Color, leadingIconContentColor: Color, trailingIconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconContentColor: Color, disabledTrailingIconContentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let assistChipColors_composable = if assistChipColors_args.is_null() {
		Composable::new(env.get_string(assistChipColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(assistChipColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &assistChipColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_assistChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let jvm = get_jvm(env);
	let assistChipElevation_composable = if assistChipElevation_args.is_null() {
		Composable::new(env.get_string(assistChipElevation_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(assistChipElevation_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &assistChipElevation_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedAssistChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let elevatedAssistChipColors_composable = if elevatedAssistChipColors_args.is_null() {
		Composable::new(env.get_string(elevatedAssistChipColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(elevatedAssistChipColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &elevatedAssistChipColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedAssistChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, labelColor: Color, leadingIconContentColor: Color, trailingIconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconContentColor: Color, disabledTrailingIconContentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let elevatedAssistChipColors_composable = if elevatedAssistChipColors_args.is_null() {
		Composable::new(env.get_string(elevatedAssistChipColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(elevatedAssistChipColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &elevatedAssistChipColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedAssistChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let jvm = get_jvm(env);
	let elevatedAssistChipElevation_composable = if elevatedAssistChipElevation_args.is_null() {
		Composable::new(env.get_string(elevatedAssistChipElevation_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(elevatedAssistChipElevation_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &elevatedAssistChipElevation_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_filterChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let filterChipColors_composable = if filterChipColors_args.is_null() {
		Composable::new(env.get_string(filterChipColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(filterChipColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &filterChipColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_filterChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, labelColor: Color, iconColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, selectedContainerColor: Color, disabledSelectedContainerColor: Color, selectedLabelColor: Color, selectedLeadingIconColor: Color, selectedTrailingIconColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let filterChipColors_composable = if filterChipColors_args.is_null() {
		Composable::new(env.get_string(filterChipColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(filterChipColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &filterChipColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_filterChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let jvm = get_jvm(env);
	let filterChipElevation_composable = if filterChipElevation_args.is_null() {
		Composable::new(env.get_string(filterChipElevation_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(filterChipElevation_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &filterChipElevation_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedFilterChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let elevatedFilterChipColors_composable = if elevatedFilterChipColors_args.is_null() {
		Composable::new(env.get_string(elevatedFilterChipColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(elevatedFilterChipColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &elevatedFilterChipColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedFilterChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, labelColor: Color, iconColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, selectedContainerColor: Color, disabledSelectedContainerColor: Color, selectedLabelColor: Color, selectedLeadingIconColor: Color, selectedTrailingIconColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let elevatedFilterChipColors_composable = if elevatedFilterChipColors_args.is_null() {
		Composable::new(env.get_string(elevatedFilterChipColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(elevatedFilterChipColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &elevatedFilterChipColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedFilterChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let jvm = get_jvm(env);
	let elevatedFilterChipElevation_composable = if elevatedFilterChipElevation_args.is_null() {
		Composable::new(env.get_string(elevatedFilterChipElevation_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(elevatedFilterChipElevation_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &elevatedFilterChipElevation_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_inputChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let inputChipColors_composable = if inputChipColors_args.is_null() {
		Composable::new(env.get_string(inputChipColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(inputChipColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &inputChipColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_inputChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, labelColor: Color, leadingIconColor: Color, trailingIconColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, selectedContainerColor: Color, disabledSelectedContainerColor: Color, selectedLabelColor: Color, selectedLeadingIconColor: Color, selectedTrailingIconColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let inputChipColors_composable = if inputChipColors_args.is_null() {
		Composable::new(env.get_string(inputChipColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(inputChipColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &inputChipColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_inputChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let jvm = get_jvm(env);
	let inputChipElevation_composable = if inputChipElevation_args.is_null() {
		Composable::new(env.get_string(inputChipElevation_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(inputChipElevation_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &inputChipElevation_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_suggestionChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let suggestionChipColors_composable = if suggestionChipColors_args.is_null() {
		Composable::new(env.get_string(suggestionChipColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(suggestionChipColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &suggestionChipColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_suggestionChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, labelColor: Color, iconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledIconContentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let suggestionChipColors_composable = if suggestionChipColors_args.is_null() {
		Composable::new(env.get_string(suggestionChipColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(suggestionChipColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &suggestionChipColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_suggestionChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let jvm = get_jvm(env);
	let suggestionChipElevation_composable = if suggestionChipElevation_args.is_null() {
		Composable::new(env.get_string(suggestionChipElevation_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(suggestionChipElevation_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &suggestionChipElevation_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedSuggestionChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let jvm = get_jvm(env);
	let elevatedSuggestionChipColors_composable = if elevatedSuggestionChipColors_args.is_null() {
		Composable::new(env.get_string(elevatedSuggestionChipColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(elevatedSuggestionChipColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &elevatedSuggestionChipColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedSuggestionChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, labelColor: Color, iconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledIconContentColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let elevatedSuggestionChipColors_composable = if elevatedSuggestionChipColors_args.is_null() {
		Composable::new(env.get_string(elevatedSuggestionChipColors_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(elevatedSuggestionChipColors_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &elevatedSuggestionChipColors_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedSuggestionChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let jvm = get_jvm(env);
	let elevatedSuggestionChipElevation_composable = if elevatedSuggestionChipElevation_args.is_null() {
		Composable::new(env.get_string(elevatedSuggestionChipElevation_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(elevatedSuggestionChipElevation_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &elevatedSuggestionChipElevation_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_DateRangePicker_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: DateRangePickerState, modifier: Modifier, dateFormatter: DatePickerFormatter, title: (@Composable () -> Unit) -> jobject {
	let jvm = get_jvm(env);
	let DateRangePicker_composable = if DateRangePicker_args.is_null() {
		Composable::new(env.get_string(DateRangePicker_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(DateRangePicker_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &DateRangePicker_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_DateRangePickerTitle_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, displayMode: DisplayMode, modifier: Modifier) -> jobject {
	let jvm = get_jvm(env);
	let DateRangePickerTitle_composable = if DateRangePickerTitle_args.is_null() {
		Composable::new(env.get_string(DateRangePickerTitle_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(DateRangePickerTitle_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &DateRangePickerTitle_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_SnackbarHost_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, hostState: SnackbarHostState, modifier: Modifier, snackbar_name: JString<'local>, snackbar_args: JString<'local>) -> jobject {
	let jvm = get_jvm(env);
	let SnackbarHost_composable = if SnackbarHost_args.is_null() {
		Composable::new(env.get_string(SnackbarHost_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(SnackbarHost_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &SnackbarHost_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_LinearProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, progress: () -> Float, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap) -> jobject {
	let jvm = get_jvm(env);
	let LinearProgressIndicator_composable = if LinearProgressIndicator_args.is_null() {
		Composable::new(env.get_string(LinearProgressIndicator_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(LinearProgressIndicator_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &LinearProgressIndicator_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_LinearProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, progress: () -> Float, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap, gapSize: Dp, drawStopIndicator: (DrawScope.() -> Unit) -> jobject {
	let jvm = get_jvm(env);
	let LinearProgressIndicator_composable = if LinearProgressIndicator_args.is_null() {
		Composable::new(env.get_string(LinearProgressIndicator_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(LinearProgressIndicator_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &LinearProgressIndicator_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_LinearProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap) -> jobject {
	let jvm = get_jvm(env);
	let LinearProgressIndicator_composable = if LinearProgressIndicator_args.is_null() {
		Composable::new(env.get_string(LinearProgressIndicator_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(LinearProgressIndicator_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &LinearProgressIndicator_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_LinearProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap, gapSize: Dp) -> jobject {
	let jvm = get_jvm(env);
	let LinearProgressIndicator_composable = if LinearProgressIndicator_args.is_null() {
		Composable::new(env.get_string(LinearProgressIndicator_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(LinearProgressIndicator_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &LinearProgressIndicator_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_LinearProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, progress: Float, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap) -> jobject {
	let jvm = get_jvm(env);
	let LinearProgressIndicator_composable = if LinearProgressIndicator_args.is_null() {
		Composable::new(env.get_string(LinearProgressIndicator_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(LinearProgressIndicator_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &LinearProgressIndicator_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_LinearProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, progress: Float, modifier: Modifier, color: Color, trackColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let LinearProgressIndicator_composable = if LinearProgressIndicator_args.is_null() {
		Composable::new(env.get_string(LinearProgressIndicator_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(LinearProgressIndicator_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &LinearProgressIndicator_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_LinearProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, color: Color, trackColor: Color) -> jobject {
	let jvm = get_jvm(env);
	let LinearProgressIndicator_composable = if LinearProgressIndicator_args.is_null() {
		Composable::new(env.get_string(LinearProgressIndicator_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(LinearProgressIndicator_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &LinearProgressIndicator_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_CircularProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, progress: () -> Float, modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap) -> jobject {
	let jvm = get_jvm(env);
	let CircularProgressIndicator_composable = if CircularProgressIndicator_args.is_null() {
		Composable::new(env.get_string(CircularProgressIndicator_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(CircularProgressIndicator_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &CircularProgressIndicator_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_CircularProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, progress: () -> Float, modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap, gapSize: Dp) -> jobject {
	let jvm = get_jvm(env);
	let CircularProgressIndicator_composable = if CircularProgressIndicator_args.is_null() {
		Composable::new(env.get_string(CircularProgressIndicator_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(CircularProgressIndicator_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &CircularProgressIndicator_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_CircularProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap) -> jobject {
	let jvm = get_jvm(env);
	let CircularProgressIndicator_composable = if CircularProgressIndicator_args.is_null() {
		Composable::new(env.get_string(CircularProgressIndicator_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(CircularProgressIndicator_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &CircularProgressIndicator_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_CircularProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, progress: Float, modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap) -> jobject {
	let jvm = get_jvm(env);
	let CircularProgressIndicator_composable = if CircularProgressIndicator_args.is_null() {
		Composable::new(env.get_string(CircularProgressIndicator_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(CircularProgressIndicator_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &CircularProgressIndicator_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_CircularProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, progress: Float, modifier: Modifier, color: Color, strokeWidth: Dp) -> jobject {
	let jvm = get_jvm(env);
	let CircularProgressIndicator_composable = if CircularProgressIndicator_args.is_null() {
		Composable::new(env.get_string(CircularProgressIndicator_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(CircularProgressIndicator_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &CircularProgressIndicator_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_CircularProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, color: Color, strokeWidth: Dp) -> jobject {
	let jvm = get_jvm(env);
	let CircularProgressIndicator_composable = if CircularProgressIndicator_args.is_null() {
		Composable::new(env.get_string(CircularProgressIndicator_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(CircularProgressIndicator_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &CircularProgressIndicator_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_HorizontalDivider_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, thickness: Dp, color: Color) -> jobject {
	let jvm = get_jvm(env);
	let HorizontalDivider_composable = if HorizontalDivider_args.is_null() {
		Composable::new(env.get_string(HorizontalDivider_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(HorizontalDivider_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &HorizontalDivider_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_VerticalDivider_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, thickness: Dp, color: Color) -> jobject {
	let jvm = get_jvm(env);
	let VerticalDivider_composable = if VerticalDivider_args.is_null() {
		Composable::new(env.get_string(VerticalDivider_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(VerticalDivider_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &VerticalDivider_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Divider_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, thickness: Dp, color: Color) -> jobject {
	let jvm = get_jvm(env);
	let Divider_composable = if Divider_args.is_null() {
		Composable::new(env.get_string(Divider_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Divider_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Divider_composable);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Indicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: PullToRefreshState, modifier: Modifier, color: Color) -> jobject {
	let jvm = get_jvm(env);
	let Indicator_composable = if Indicator_args.is_null() {
		Composable::new(env.get_string(Indicator_name).expect("Couldn't get Java string").to_str().unwrap(), vec![])
	} else {
		let args_str = env.get_string(Indicator_args).expect("Couldn't get Java string").to_str().unwrap();
		let args_json: Vec<Composable> = serde_json::from_str(args_str).expect("Couldn't parse JSON");
		args_json[0].clone()
	};
	// Call JNI function with the Composable instance
	call_jni_function(&jvm, &Indicator_composable);
}

