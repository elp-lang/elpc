use jni::{objects::JClass,sys::jobject,JavaVM,JNIEnv};

use crate::types::{self, call_composable_function, Unit, Composable};

// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Label.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Label_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, label: Composable, modifier: Modifier, interactionSource: MutableInteractionSource, isPersistent: Boolean, content: Composable) -> jobject {
	let args = vec![
		label.to_jni_value(),
		modifier.to_jni_value(),
		interactionSource.to_jni_value(),
		isPersistent.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Text.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Text_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, text: String, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, minLines: Int, onTextLayout: fn() -> Unit, style: TextStyle) -> jobject {
	let args = vec![
		text.to_jni_value(),
		modifier.to_jni_value(),
		color.to_jni_value(),
		fontSize.to_jni_value(),
		fontStyle.to_jni_value(),
		fontWeight.to_jni_value(),
		fontFamily.to_jni_value(),
		letterSpacing.to_jni_value(),
		textDecoration.to_jni_value(),
		textAlign.to_jni_value(),
		lineHeight.to_jni_value(),
		overflow.to_jni_value(),
		softWrap.to_jni_value(),
		maxLines.to_jni_value(),
		minLines.to_jni_value(),
		onTextLayout.to_jni_value(),
		style.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Text.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Text_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, text: String, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, onTextLayout: fn() -> Unit, style: TextStyle) -> jobject {
	let args = vec![
		text.to_jni_value(),
		modifier.to_jni_value(),
		color.to_jni_value(),
		fontSize.to_jni_value(),
		fontStyle.to_jni_value(),
		fontWeight.to_jni_value(),
		fontFamily.to_jni_value(),
		letterSpacing.to_jni_value(),
		textDecoration.to_jni_value(),
		textAlign.to_jni_value(),
		lineHeight.to_jni_value(),
		overflow.to_jni_value(),
		softWrap.to_jni_value(),
		maxLines.to_jni_value(),
		onTextLayout.to_jni_value(),
		style.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Text.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Text_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, text: AnnotatedString, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, minLines: Int, inlineContent: Map<String, InlineTextContent>, onTextLayout: fn() -> Unit, style: TextStyle) -> jobject {
	let args = vec![
		text.to_jni_value(),
		modifier.to_jni_value(),
		color.to_jni_value(),
		fontSize.to_jni_value(),
		fontStyle.to_jni_value(),
		fontWeight.to_jni_value(),
		fontFamily.to_jni_value(),
		letterSpacing.to_jni_value(),
		textDecoration.to_jni_value(),
		textAlign.to_jni_value(),
		lineHeight.to_jni_value(),
		overflow.to_jni_value(),
		softWrap.to_jni_value(),
		maxLines.to_jni_value(),
		minLines.to_jni_value(),
		inlineContent.to_jni_value(),
		onTextLayout.to_jni_value(),
		style.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Text.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Text_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, text: AnnotatedString, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, inlineContent: Map<String, InlineTextContent>, onTextLayout: fn() -> Unit, style: TextStyle) -> jobject {
	let args = vec![
		text.to_jni_value(),
		modifier.to_jni_value(),
		color.to_jni_value(),
		fontSize.to_jni_value(),
		fontStyle.to_jni_value(),
		fontWeight.to_jni_value(),
		fontFamily.to_jni_value(),
		letterSpacing.to_jni_value(),
		textDecoration.to_jni_value(),
		textAlign.to_jni_value(),
		lineHeight.to_jni_value(),
		overflow.to_jni_value(),
		softWrap.to_jni_value(),
		maxLines.to_jni_value(),
		inlineContent.to_jni_value(),
		onTextLayout.to_jni_value(),
		style.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Text.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_ProvideTextStyle_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: TextStyle, content: Composable) -> jobject {
	let args = vec![
		value.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/DatePicker.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_DatePicker_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: DatePickerState, modifier: Modifier, dateFormatter: DatePickerFormatter, title: fn() -> Unit, headline: fn() -> Unit, showModeToggle: Boolean, colors: DatePickerColors) -> jobject {
	let args = vec![
		state.to_jni_value(),
		modifier.to_jni_value(),
		dateFormatter.to_jni_value(),
		title.to_jni_value(),
		headline.to_jni_value(),
		showModeToggle.to_jni_value(),
		colors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/DatePicker.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, titleContentColor: Color, headlineContentColor: Color, weekdayContentColor: Color, subheadContentColor: Color, navigationContentColor: Color, yearContentColor: Color, disabledYearContentColor: Color, currentYearContentColor: Color, selectedYearContentColor: Color, disabledSelectedYearContentColor: Color, selectedYearContainerColor: Color, disabledSelectedYearContainerColor: Color, dayContentColor: Color, disabledDayContentColor: Color, selectedDayContentColor: Color, disabledSelectedDayContentColor: Color, selectedDayContainerColor: Color, disabledSelectedDayContainerColor: Color, todayContentColor: Color, todayDateBorderColor: Color, dayInSelectionRangeContentColor: Color, dayInSelectionRangeContainerColor: Color, dividerColor: Color, dateTextFieldColors: TextFieldColors, defaultDatePickerColors: Composable) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		titleContentColor.to_jni_value(),
		headlineContentColor.to_jni_value(),
		weekdayContentColor.to_jni_value(),
		subheadContentColor.to_jni_value(),
		navigationContentColor.to_jni_value(),
		yearContentColor.to_jni_value(),
		disabledYearContentColor.to_jni_value(),
		currentYearContentColor.to_jni_value(),
		selectedYearContentColor.to_jni_value(),
		disabledSelectedYearContentColor.to_jni_value(),
		selectedYearContainerColor.to_jni_value(),
		disabledSelectedYearContainerColor.to_jni_value(),
		dayContentColor.to_jni_value(),
		disabledDayContentColor.to_jni_value(),
		selectedDayContentColor.to_jni_value(),
		disabledSelectedDayContentColor.to_jni_value(),
		selectedDayContainerColor.to_jni_value(),
		disabledSelectedDayContainerColor.to_jni_value(),
		todayContentColor.to_jni_value(),
		todayDateBorderColor.to_jni_value(),
		dayInSelectionRangeContentColor.to_jni_value(),
		dayInSelectionRangeContainerColor.to_jni_value(),
		dividerColor.to_jni_value(),
		dateTextFieldColors.to_jni_value(),
		defaultDatePickerColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/DatePicker.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_DatePickerTitle_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, displayMode: DisplayMode, modifier: Modifier) -> jobject {
	let args = vec![
		displayMode.to_jni_value(),
		modifier.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/DatePicker.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_DatePickerHeadline_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedDateMillis: Long, displayMode: DisplayMode, dateFormatter: DatePickerFormatter, modifier: Modifier) -> jobject {
	let args = vec![
		selectedDateMillis.to_jni_value(),
		displayMode.to_jni_value(),
		dateFormatter.to_jni_value(),
		modifier.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TextFieldImpl.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Transition_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, inputState: InputPhase, focusedTextStyleColor: Color, unfocusedTextStyleColor: Color, contentColor: Composable, showLabel: Boolean, content: Composable, labelTextStyleColor: Color, labelContentColor: Color, placeholderOpacity: Float, prefixSuffixOpacity: Float) -> jobject {
	let args = vec![
		inputState.to_jni_value(),
		focusedTextStyleColor.to_jni_value(),
		unfocusedTextStyleColor.to_jni_value(),
		contentColor.to_jni_value(),
		showLabel.to_jni_value(),
		content.to_jni_value(),
		labelTextStyleColor.to_jni_value(),
		labelContentColor.to_jni_value(),
		placeholderOpacity.to_jni_value(),
		prefixSuffixOpacity.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Scaffold.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Scaffold_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, topBar: Composable, bottomBar: Composable, snackbarHost: Composable, floatingActionButton: Composable, floatingActionButtonPosition: FabPosition, containerColor: Color, contentColor: Color, contentWindowInsets: WindowInsets, content: Composable) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		topBar.to_jni_value(),
		bottomBar.to_jni_value(),
		snackbarHost.to_jni_value(),
		floatingActionButton.to_jni_value(),
		floatingActionButtonPosition.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		contentWindowInsets.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TimePicker.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, clockDialColor: Color, clockDialSelectedContentColor: Color, clockDialUnselectedContentColor: Color, selectorColor: Color, containerColor: Color, periodSelectorBorderColor: Color, periodSelectorSelectedContainerColor: Color, periodSelectorUnselectedContainerColor: Color, periodSelectorSelectedContentColor: Color, periodSelectorUnselectedContentColor: Color, timeSelectorSelectedContainerColor: Color, timeSelectorUnselectedContainerColor: Color, timeSelectorSelectedContentColor: Color, timeSelectorUnselectedContentColor: Color, defaultTimePickerColors: TimePickerColors
        get() -> jobject {
	let args = vec![
		clockDialColor.to_jni_value(),
		clockDialSelectedContentColor.to_jni_value(),
		clockDialUnselectedContentColor.to_jni_value(),
		selectorColor.to_jni_value(),
		containerColor.to_jni_value(),
		periodSelectorBorderColor.to_jni_value(),
		periodSelectorSelectedContainerColor.to_jni_value(),
		periodSelectorUnselectedContainerColor.to_jni_value(),
		periodSelectorSelectedContentColor.to_jni_value(),
		periodSelectorUnselectedContentColor.to_jni_value(),
		timeSelectorSelectedContainerColor.to_jni_value(),
		timeSelectorUnselectedContainerColor.to_jni_value(),
		timeSelectorSelectedContentColor.to_jni_value(),
		timeSelectorUnselectedContentColor.to_jni_value(),
		defaultTimePickerColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TimePicker.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_layoutType_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, clockDialColor: Color, selectorColor: Color, containerColor: Color, periodSelectorBorderColor: Color, clockDialSelectedContentColor: Color, clockDialUnselectedContentColor: Color, periodSelectorSelectedContainerColor: Color, periodSelectorUnselectedContainerColor: Color, periodSelectorSelectedContentColor: Color, periodSelectorUnselectedContentColor: Color, timeSelectorSelectedContainerColor: Color, timeSelectorUnselectedContainerColor: Color, timeSelectorSelectedContentColor: Color, timeSelectorUnselectedContentColor: Color) -> jobject {
	let args = vec![
		clockDialColor.to_jni_value(),
		selectorColor.to_jni_value(),
		containerColor.to_jni_value(),
		periodSelectorBorderColor.to_jni_value(),
		clockDialSelectedContentColor.to_jni_value(),
		clockDialUnselectedContentColor.to_jni_value(),
		periodSelectorSelectedContainerColor.to_jni_value(),
		periodSelectorUnselectedContainerColor.to_jni_value(),
		periodSelectorSelectedContentColor.to_jni_value(),
		periodSelectorUnselectedContentColor.to_jni_value(),
		timeSelectorSelectedContainerColor.to_jni_value(),
		timeSelectorUnselectedContainerColor.to_jni_value(),
		timeSelectorSelectedContentColor.to_jni_value(),
		timeSelectorUnselectedContentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Menu.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_itemColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, textColor: Color, leadingIconColor: Color, trailingIconColor: Color, disabledTextColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, defaultMenuItemColors: MenuItemColors
        get() -> jobject {
	let args = vec![
		textColor.to_jni_value(),
		leadingIconColor.to_jni_value(),
		trailingIconColor.to_jni_value(),
		disabledTextColor.to_jni_value(),
		disabledLeadingIconColor.to_jni_value(),
		disabledTrailingIconColor.to_jni_value(),
		defaultMenuItemColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Slider.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Slider_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: Float, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, valueRange: ClosedFloatingPointRange<Float>, steps: Int, onValueChangeFinished: fn() -> Unit, colors: SliderColors, interactionSource: MutableInteractionSource) -> jobject {
	let args = vec![
		value.to_jni_value(),
		onValueChange.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		valueRange.to_jni_value(),
		steps.to_jni_value(),
		onValueChangeFinished.to_jni_value(),
		colors.to_jni_value(),
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Slider.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_RangeSlider_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: ClosedFloatingPointRange<Float>, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, valueRange: ClosedFloatingPointRange<Float>, steps: Int, onValueChangeFinished: fn() -> Unit, colors: SliderColors) -> jobject {
	let args = vec![
		value.to_jni_value(),
		onValueChange.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		valueRange.to_jni_value(),
		steps.to_jni_value(),
		onValueChangeFinished.to_jni_value(),
		colors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Slider.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, thumbColor: Color, activeTrackColor: Color, activeTickColor: Color, inactiveTrackColor: Color, inactiveTickColor: Color, disabledThumbColor: Color, disabledActiveTrackColor: Color, disabledActiveTickColor: Color, disabledInactiveTrackColor: Color, disabledInactiveTickColor: Color, defaultSliderColors: SliderColors
        get() -> jobject {
	let args = vec![
		thumbColor.to_jni_value(),
		activeTrackColor.to_jni_value(),
		activeTickColor.to_jni_value(),
		inactiveTrackColor.to_jni_value(),
		inactiveTickColor.to_jni_value(),
		disabledThumbColor.to_jni_value(),
		disabledActiveTrackColor.to_jni_value(),
		disabledActiveTickColor.to_jni_value(),
		disabledInactiveTrackColor.to_jni_value(),
		disabledInactiveTickColor.to_jni_value(),
		defaultSliderColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Slider.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Thumb_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, interactionSource: MutableInteractionSource, modifier: Modifier, colors: SliderColors, enabled: Boolean, thumbSize: DpSize) -> jobject {
	let args = vec![
		interactionSource.to_jni_value(),
		modifier.to_jni_value(),
		colors.to_jni_value(),
		enabled.to_jni_value(),
		thumbSize.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Slider.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Track_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, rangeSliderState: RangeSliderState, modifier: Modifier, colors: SliderColors, enabled: Boolean) -> jobject {
	let args = vec![
		rangeSliderState.to_jni_value(),
		modifier.to_jni_value(),
		colors.to_jni_value(),
		enabled.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Card.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Card_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, content: Composable) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Card.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Card_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
	let args = vec![
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Card.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_ElevatedCard_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, shape: Shape, colors: CardColors, elevation: CardElevation, content: Composable, https: //m3, https: //developer, also: * [Surface], onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: CardColors, elevation: CardElevation, interactionSource: MutableInteractionSource, content: Composable, https: //m3, also: * [Surface], shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, content: Composable, https: //m3, https: //developer, also: * [Surface], onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, interactionSource: MutableInteractionSource, content: Composable, shape: Composable, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, defaultCardColors: CardColors
        get() -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		content.to_jni_value(),
		https.to_jni_value(),
		https.to_jni_value(),
		also.to_jni_value(),
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value(),
		https.to_jni_value(),
		also.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		content.to_jni_value(),
		https.to_jni_value(),
		https.to_jni_value(),
		also.to_jni_value(),
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value(),
		shape.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value(),
		defaultElevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value(),
		defaultElevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value(),
		defaultCardColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Card.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedCardColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, defaultElevatedCardColors: CardColors
        get() -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value(),
		defaultElevatedCardColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Card.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_outlinedCardColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, defaultOutlinedCardColors: CardColors
        get() -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value(),
		defaultOutlinedCardColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Card.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_outlinedCardBorder_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, enabled: Boolean) -> jobject {
	let args = vec![
		enabled.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Button.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Button_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
	let args = vec![
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		contentPadding.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Button.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_ElevatedButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: Composable, https: //m3, https: //developer, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: Composable, https: //m3, https: //developer, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: Composable, https: //m3, https: //developer, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: Composable, shape: Composable, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, defaultButtonColors: ButtonColors
        get() -> jobject {
	let args = vec![
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		contentPadding.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value(),
		https.to_jni_value(),
		https.to_jni_value(),
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		contentPadding.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value(),
		https.to_jni_value(),
		https.to_jni_value(),
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		contentPadding.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value(),
		https.to_jni_value(),
		https.to_jni_value(),
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		contentPadding.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value(),
		shape.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value(),
		defaultButtonColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Button.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, defaultElevatedButtonColors: ButtonColors
        get() -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value(),
		defaultElevatedButtonColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Button.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_filledTonalButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, defaultFilledTonalButtonColors: ButtonColors
        get() -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value(),
		defaultFilledTonalButtonColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Button.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_outlinedButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, defaultOutlinedButtonColors: ButtonColors
        get() -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value(),
		defaultOutlinedButtonColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Button.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_textButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, defaultTextButtonColors: ButtonColors
        get() -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value(),
		defaultTextButtonColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Button.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_buttonElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp, outlinedButtonBorder: Composable, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp) -> jobject {
	let args = vec![
		defaultElevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		disabledElevation.to_jni_value(),
		defaultElevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		disabledElevation.to_jni_value(),
		defaultElevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		disabledElevation.to_jni_value(),
		outlinedButtonBorder.to_jni_value(),
		defaultElevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		disabledElevation.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/NavigationBar.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_NavigationBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, containerColor: Color, contentColor: Color, tonalElevation: Dp, windowInsets: WindowInsets, content: Composable) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		tonalElevation.to_jni_value(),
		windowInsets.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/NavigationBar.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, disabledIconColor: Color, disabledTextColor: Color, defaultNavigationBarItemColors: NavigationBarItemColors
        get() -> jobject {
	let args = vec![
		selectedIconColor.to_jni_value(),
		selectedTextColor.to_jni_value(),
		indicatorColor.to_jni_value(),
		unselectedIconColor.to_jni_value(),
		unselectedTextColor.to_jni_value(),
		disabledIconColor.to_jni_value(),
		disabledTextColor.to_jni_value(),
		defaultNavigationBarItemColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/NavigationBar.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, selectedIconColor: Color, selectedTextColor: Color, selectedIndicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, disabledIconColor: Color, disabledTextColor: Color) -> jobject {
	let args = vec![
		selectedIconColor.to_jni_value(),
		selectedTextColor.to_jni_value(),
		indicatorColor.to_jni_value(),
		unselectedIconColor.to_jni_value(),
		unselectedTextColor.to_jni_value(),
		selectedIconColor.to_jni_value(),
		selectedTextColor.to_jni_value(),
		selectedIndicatorColor.to_jni_value(),
		unselectedIconColor.to_jni_value(),
		unselectedTextColor.to_jni_value(),
		disabledIconColor.to_jni_value(),
		disabledTextColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/MaterialTheme.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_MaterialTheme_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, colorScheme: ColorScheme, shapes: Shapes, typography: Typography, content: Composable) -> jobject {
	let args = vec![
		colorScheme.to_jni_value(),
		shapes.to_jni_value(),
		typography.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Checkbox.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Checkbox_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, checked: Boolean, onCheckedChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, colors: CheckboxColors, interactionSource: MutableInteractionSource) -> jobject {
	let args = vec![
		checked.to_jni_value(),
		onCheckedChange.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		colors.to_jni_value(),
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Checkbox.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_TriStateCheckbox_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: ToggleableState, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, colors: CheckboxColors, interactionSource: MutableInteractionSource) -> jobject {
	let args = vec![
		state.to_jni_value(),
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		colors.to_jni_value(),
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Checkbox.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, checkedColor: Color, uncheckedColor: Color, checkmarkColor: Color, disabledCheckedColor: Color, disabledUncheckedColor: Color, disabledIndeterminateColor: Color, defaultCheckboxColors: CheckboxColors
        get() -> jobject {
	let args = vec![
		checkedColor.to_jni_value(),
		uncheckedColor.to_jni_value(),
		checkmarkColor.to_jni_value(),
		disabledCheckedColor.to_jni_value(),
		disabledUncheckedColor.to_jni_value(),
		disabledIndeterminateColor.to_jni_value(),
		defaultCheckboxColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TextField.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_TextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: String, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, prefix: Composable, suffix: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
	let args = vec![
		value.to_jni_value(),
		onValueChange.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		readOnly.to_jni_value(),
		textStyle.to_jni_value(),
		label.to_jni_value(),
		placeholder.to_jni_value(),
		leadingIcon.to_jni_value(),
		trailingIcon.to_jni_value(),
		prefix.to_jni_value(),
		suffix.to_jni_value(),
		supportingText.to_jni_value(),
		isError.to_jni_value(),
		visualTransformation.to_jni_value(),
		keyboardOptions.to_jni_value(),
		keyboardActions.to_jni_value(),
		singleLine.to_jni_value(),
		maxLines.to_jni_value(),
		minLines.to_jni_value(),
		interactionSource.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TextField.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_TextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: TextFieldValue, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, prefix: Composable, suffix: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
	let args = vec![
		value.to_jni_value(),
		onValueChange.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		readOnly.to_jni_value(),
		textStyle.to_jni_value(),
		label.to_jni_value(),
		placeholder.to_jni_value(),
		leadingIcon.to_jni_value(),
		trailingIcon.to_jni_value(),
		prefix.to_jni_value(),
		suffix.to_jni_value(),
		supportingText.to_jni_value(),
		isError.to_jni_value(),
		visualTransformation.to_jni_value(),
		keyboardOptions.to_jni_value(),
		keyboardActions.to_jni_value(),
		singleLine.to_jni_value(),
		maxLines.to_jni_value(),
		minLines.to_jni_value(),
		interactionSource.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TextField.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_TextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: String, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
	let args = vec![
		value.to_jni_value(),
		onValueChange.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		readOnly.to_jni_value(),
		textStyle.to_jni_value(),
		label.to_jni_value(),
		placeholder.to_jni_value(),
		leadingIcon.to_jni_value(),
		trailingIcon.to_jni_value(),
		supportingText.to_jni_value(),
		isError.to_jni_value(),
		visualTransformation.to_jni_value(),
		keyboardOptions.to_jni_value(),
		keyboardActions.to_jni_value(),
		singleLine.to_jni_value(),
		maxLines.to_jni_value(),
		minLines.to_jni_value(),
		interactionSource.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TextField.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_TextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: TextFieldValue, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
	let args = vec![
		value.to_jni_value(),
		onValueChange.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		readOnly.to_jni_value(),
		textStyle.to_jni_value(),
		label.to_jni_value(),
		placeholder.to_jni_value(),
		leadingIcon.to_jni_value(),
		trailingIcon.to_jni_value(),
		supportingText.to_jni_value(),
		isError.to_jni_value(),
		visualTransformation.to_jni_value(),
		keyboardOptions.to_jni_value(),
		keyboardActions.to_jni_value(),
		singleLine.to_jni_value(),
		maxLines.to_jni_value(),
		minLines.to_jni_value(),
		interactionSource.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TextFieldDefaults.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_ContainerBox_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape) -> jobject {
	let args = vec![
		enabled.to_jni_value(),
		isError.to_jni_value(),
		interactionSource.to_jni_value(),
		colors.to_jni_value(),
		shape.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TextFieldDefaults.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, focusedTextColor: Color, unfocusedTextColor: Color, disabledTextColor: Color, errorTextColor: Color, focusedContainerColor: Color, unfocusedContainerColor: Color, disabledContainerColor: Color, errorContainerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedIndicatorColor: Color, unfocusedIndicatorColor: Color, disabledIndicatorColor: Color, errorIndicatorColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, focusedPlaceholderColor: Color, unfocusedPlaceholderColor: Color, disabledPlaceholderColor: Color, errorPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color, defaultTextFieldColors: Composable) -> jobject {
	let args = vec![
		focusedTextColor.to_jni_value(),
		unfocusedTextColor.to_jni_value(),
		disabledTextColor.to_jni_value(),
		errorTextColor.to_jni_value(),
		focusedContainerColor.to_jni_value(),
		unfocusedContainerColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		errorContainerColor.to_jni_value(),
		cursorColor.to_jni_value(),
		errorCursorColor.to_jni_value(),
		selectionColors.to_jni_value(),
		focusedIndicatorColor.to_jni_value(),
		unfocusedIndicatorColor.to_jni_value(),
		disabledIndicatorColor.to_jni_value(),
		errorIndicatorColor.to_jni_value(),
		focusedLeadingIconColor.to_jni_value(),
		unfocusedLeadingIconColor.to_jni_value(),
		disabledLeadingIconColor.to_jni_value(),
		errorLeadingIconColor.to_jni_value(),
		focusedTrailingIconColor.to_jni_value(),
		unfocusedTrailingIconColor.to_jni_value(),
		disabledTrailingIconColor.to_jni_value(),
		errorTrailingIconColor.to_jni_value(),
		focusedLabelColor.to_jni_value(),
		unfocusedLabelColor.to_jni_value(),
		disabledLabelColor.to_jni_value(),
		errorLabelColor.to_jni_value(),
		focusedPlaceholderColor.to_jni_value(),
		unfocusedPlaceholderColor.to_jni_value(),
		disabledPlaceholderColor.to_jni_value(),
		errorPlaceholderColor.to_jni_value(),
		focusedSupportingTextColor.to_jni_value(),
		unfocusedSupportingTextColor.to_jni_value(),
		disabledSupportingTextColor.to_jni_value(),
		errorSupportingTextColor.to_jni_value(),
		focusedPrefixColor.to_jni_value(),
		unfocusedPrefixColor.to_jni_value(),
		disabledPrefixColor.to_jni_value(),
		errorPrefixColor.to_jni_value(),
		focusedSuffixColor.to_jni_value(),
		unfocusedSuffixColor.to_jni_value(),
		disabledSuffixColor.to_jni_value(),
		errorSuffixColor.to_jni_value(),
		defaultTextFieldColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TextFieldDefaults.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_FilledContainerBox_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape, enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape, focusedBorderThickness: Dp, unfocusedBorderThickness: Dp, start: Dp, end: Dp, top: Dp, bottom: Dp, start: Dp, top: Dp, end: Dp, bottom: Dp, start: Dp, top: Dp, end: Dp, bottom: Dp, focusedTextColor: Color, unfocusedTextColor: Color, disabledTextColor: Color, errorTextColor: Color, containerColor: Color, errorContainerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedIndicatorColor: Color, unfocusedIndicatorColor: Color, disabledIndicatorColor: Color, errorIndicatorColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, focusedPlaceholderColor: Color, unfocusedPlaceholderColor: Color, disabledPlaceholderColor: Color, errorPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color, focusedTextColor: Color, unfocusedTextColor: Color, disabledTextColor: Color, errorTextColor: Color, containerColor: Color, errorContainerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedBorderColor: Color, unfocusedBorderColor: Color, disabledBorderColor: Color, errorBorderColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, focusedPlaceholderColor: Color, unfocusedPlaceholderColor: Color, disabledPlaceholderColor: Color, errorPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color, value: String, innerTextField: Composable, enabled: Boolean, singleLine: Boolean, visualTransformation: VisualTransformation, interactionSource: InteractionSource, isError: Boolean, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, prefix: Composable, suffix: Composable, supportingText: Composable, shape: Shape, colors: TextFieldColors, contentPadding: PaddingValues) -> jobject {
	let args = vec![
		enabled.to_jni_value(),
		isError.to_jni_value(),
		interactionSource.to_jni_value(),
		colors.to_jni_value(),
		shape.to_jni_value(),
		enabled.to_jni_value(),
		isError.to_jni_value(),
		interactionSource.to_jni_value(),
		colors.to_jni_value(),
		shape.to_jni_value(),
		focusedBorderThickness.to_jni_value(),
		unfocusedBorderThickness.to_jni_value(),
		start.to_jni_value(),
		end.to_jni_value(),
		top.to_jni_value(),
		bottom.to_jni_value(),
		start.to_jni_value(),
		top.to_jni_value(),
		end.to_jni_value(),
		bottom.to_jni_value(),
		start.to_jni_value(),
		top.to_jni_value(),
		end.to_jni_value(),
		bottom.to_jni_value(),
		focusedTextColor.to_jni_value(),
		unfocusedTextColor.to_jni_value(),
		disabledTextColor.to_jni_value(),
		errorTextColor.to_jni_value(),
		containerColor.to_jni_value(),
		errorContainerColor.to_jni_value(),
		cursorColor.to_jni_value(),
		errorCursorColor.to_jni_value(),
		selectionColors.to_jni_value(),
		focusedIndicatorColor.to_jni_value(),
		unfocusedIndicatorColor.to_jni_value(),
		disabledIndicatorColor.to_jni_value(),
		errorIndicatorColor.to_jni_value(),
		focusedLeadingIconColor.to_jni_value(),
		unfocusedLeadingIconColor.to_jni_value(),
		disabledLeadingIconColor.to_jni_value(),
		errorLeadingIconColor.to_jni_value(),
		focusedTrailingIconColor.to_jni_value(),
		unfocusedTrailingIconColor.to_jni_value(),
		disabledTrailingIconColor.to_jni_value(),
		errorTrailingIconColor.to_jni_value(),
		focusedLabelColor.to_jni_value(),
		unfocusedLabelColor.to_jni_value(),
		disabledLabelColor.to_jni_value(),
		errorLabelColor.to_jni_value(),
		focusedPlaceholderColor.to_jni_value(),
		unfocusedPlaceholderColor.to_jni_value(),
		disabledPlaceholderColor.to_jni_value(),
		errorPlaceholderColor.to_jni_value(),
		focusedSupportingTextColor.to_jni_value(),
		unfocusedSupportingTextColor.to_jni_value(),
		disabledSupportingTextColor.to_jni_value(),
		errorSupportingTextColor.to_jni_value(),
		focusedPrefixColor.to_jni_value(),
		unfocusedPrefixColor.to_jni_value(),
		disabledPrefixColor.to_jni_value(),
		errorPrefixColor.to_jni_value(),
		focusedSuffixColor.to_jni_value(),
		unfocusedSuffixColor.to_jni_value(),
		disabledSuffixColor.to_jni_value(),
		errorSuffixColor.to_jni_value(),
		focusedTextColor.to_jni_value(),
		unfocusedTextColor.to_jni_value(),
		disabledTextColor.to_jni_value(),
		errorTextColor.to_jni_value(),
		containerColor.to_jni_value(),
		errorContainerColor.to_jni_value(),
		cursorColor.to_jni_value(),
		errorCursorColor.to_jni_value(),
		selectionColors.to_jni_value(),
		focusedBorderColor.to_jni_value(),
		unfocusedBorderColor.to_jni_value(),
		disabledBorderColor.to_jni_value(),
		errorBorderColor.to_jni_value(),
		focusedLeadingIconColor.to_jni_value(),
		unfocusedLeadingIconColor.to_jni_value(),
		disabledLeadingIconColor.to_jni_value(),
		errorLeadingIconColor.to_jni_value(),
		focusedTrailingIconColor.to_jni_value(),
		unfocusedTrailingIconColor.to_jni_value(),
		disabledTrailingIconColor.to_jni_value(),
		errorTrailingIconColor.to_jni_value(),
		focusedLabelColor.to_jni_value(),
		unfocusedLabelColor.to_jni_value(),
		disabledLabelColor.to_jni_value(),
		errorLabelColor.to_jni_value(),
		focusedPlaceholderColor.to_jni_value(),
		unfocusedPlaceholderColor.to_jni_value(),
		disabledPlaceholderColor.to_jni_value(),
		errorPlaceholderColor.to_jni_value(),
		focusedSupportingTextColor.to_jni_value(),
		unfocusedSupportingTextColor.to_jni_value(),
		disabledSupportingTextColor.to_jni_value(),
		errorSupportingTextColor.to_jni_value(),
		focusedPrefixColor.to_jni_value(),
		unfocusedPrefixColor.to_jni_value(),
		disabledPrefixColor.to_jni_value(),
		errorPrefixColor.to_jni_value(),
		focusedSuffixColor.to_jni_value(),
		unfocusedSuffixColor.to_jni_value(),
		disabledSuffixColor.to_jni_value(),
		errorSuffixColor.to_jni_value(),
		value.to_jni_value(),
		innerTextField.to_jni_value(),
		enabled.to_jni_value(),
		singleLine.to_jni_value(),
		visualTransformation.to_jni_value(),
		interactionSource.to_jni_value(),
		isError.to_jni_value(),
		label.to_jni_value(),
		placeholder.to_jni_value(),
		leadingIcon.to_jni_value(),
		trailingIcon.to_jni_value(),
		prefix.to_jni_value(),
		suffix.to_jni_value(),
		supportingText.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		contentPadding.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TextFieldDefaults.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_textFieldColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, textColor: Color, disabledTextColor: Color, containerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedIndicatorColor: Color, unfocusedIndicatorColor: Color, disabledIndicatorColor: Color, errorIndicatorColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, placeholderColor: Color, disabledPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color, textColor: Color, disabledTextColor: Color, containerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedBorderColor: Color, unfocusedBorderColor: Color, disabledBorderColor: Color, errorBorderColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, placeholderColor: Color, disabledPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color, value: String, innerTextField: Composable, enabled: Boolean, singleLine: Boolean, visualTransformation: VisualTransformation, interactionSource: InteractionSource, isError: Boolean, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, supportingText: Composable, shape: Shape, colors: TextFieldColors, contentPadding: PaddingValues) -> jobject {
	let args = vec![
		textColor.to_jni_value(),
		disabledTextColor.to_jni_value(),
		containerColor.to_jni_value(),
		cursorColor.to_jni_value(),
		errorCursorColor.to_jni_value(),
		selectionColors.to_jni_value(),
		focusedIndicatorColor.to_jni_value(),
		unfocusedIndicatorColor.to_jni_value(),
		disabledIndicatorColor.to_jni_value(),
		errorIndicatorColor.to_jni_value(),
		focusedLeadingIconColor.to_jni_value(),
		unfocusedLeadingIconColor.to_jni_value(),
		disabledLeadingIconColor.to_jni_value(),
		errorLeadingIconColor.to_jni_value(),
		focusedTrailingIconColor.to_jni_value(),
		unfocusedTrailingIconColor.to_jni_value(),
		disabledTrailingIconColor.to_jni_value(),
		errorTrailingIconColor.to_jni_value(),
		focusedLabelColor.to_jni_value(),
		unfocusedLabelColor.to_jni_value(),
		disabledLabelColor.to_jni_value(),
		errorLabelColor.to_jni_value(),
		placeholderColor.to_jni_value(),
		disabledPlaceholderColor.to_jni_value(),
		focusedSupportingTextColor.to_jni_value(),
		unfocusedSupportingTextColor.to_jni_value(),
		disabledSupportingTextColor.to_jni_value(),
		errorSupportingTextColor.to_jni_value(),
		focusedPrefixColor.to_jni_value(),
		unfocusedPrefixColor.to_jni_value(),
		disabledPrefixColor.to_jni_value(),
		errorPrefixColor.to_jni_value(),
		focusedSuffixColor.to_jni_value(),
		unfocusedSuffixColor.to_jni_value(),
		disabledSuffixColor.to_jni_value(),
		errorSuffixColor.to_jni_value(),
		textColor.to_jni_value(),
		disabledTextColor.to_jni_value(),
		containerColor.to_jni_value(),
		cursorColor.to_jni_value(),
		errorCursorColor.to_jni_value(),
		selectionColors.to_jni_value(),
		focusedBorderColor.to_jni_value(),
		unfocusedBorderColor.to_jni_value(),
		disabledBorderColor.to_jni_value(),
		errorBorderColor.to_jni_value(),
		focusedLeadingIconColor.to_jni_value(),
		unfocusedLeadingIconColor.to_jni_value(),
		disabledLeadingIconColor.to_jni_value(),
		errorLeadingIconColor.to_jni_value(),
		focusedTrailingIconColor.to_jni_value(),
		unfocusedTrailingIconColor.to_jni_value(),
		disabledTrailingIconColor.to_jni_value(),
		errorTrailingIconColor.to_jni_value(),
		focusedLabelColor.to_jni_value(),
		unfocusedLabelColor.to_jni_value(),
		disabledLabelColor.to_jni_value(),
		errorLabelColor.to_jni_value(),
		placeholderColor.to_jni_value(),
		disabledPlaceholderColor.to_jni_value(),
		focusedSupportingTextColor.to_jni_value(),
		unfocusedSupportingTextColor.to_jni_value(),
		disabledSupportingTextColor.to_jni_value(),
		errorSupportingTextColor.to_jni_value(),
		focusedPrefixColor.to_jni_value(),
		unfocusedPrefixColor.to_jni_value(),
		disabledPrefixColor.to_jni_value(),
		errorPrefixColor.to_jni_value(),
		focusedSuffixColor.to_jni_value(),
		unfocusedSuffixColor.to_jni_value(),
		disabledSuffixColor.to_jni_value(),
		errorSuffixColor.to_jni_value(),
		value.to_jni_value(),
		innerTextField.to_jni_value(),
		enabled.to_jni_value(),
		singleLine.to_jni_value(),
		visualTransformation.to_jni_value(),
		interactionSource.to_jni_value(),
		isError.to_jni_value(),
		label.to_jni_value(),
		placeholder.to_jni_value(),
		leadingIcon.to_jni_value(),
		trailingIcon.to_jni_value(),
		supportingText.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		contentPadding.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TextFieldDefaults.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_ContainerBox_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape, focusedBorderThickness: Dp, unfocusedBorderThickness: Dp) -> jobject {
	let args = vec![
		enabled.to_jni_value(),
		isError.to_jni_value(),
		interactionSource.to_jni_value(),
		colors.to_jni_value(),
		shape.to_jni_value(),
		focusedBorderThickness.to_jni_value(),
		unfocusedBorderThickness.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TextFieldDefaults.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, focusedTextColor: Color, unfocusedTextColor: Color, disabledTextColor: Color, errorTextColor: Color, focusedContainerColor: Color, unfocusedContainerColor: Color, disabledContainerColor: Color, errorContainerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedBorderColor: Color, unfocusedBorderColor: Color, disabledBorderColor: Color, errorBorderColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, focusedPlaceholderColor: Color, unfocusedPlaceholderColor: Color, disabledPlaceholderColor: Color, errorPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color, defaultOutlinedTextFieldColors: Composable) -> jobject {
	let args = vec![
		focusedTextColor.to_jni_value(),
		unfocusedTextColor.to_jni_value(),
		disabledTextColor.to_jni_value(),
		errorTextColor.to_jni_value(),
		focusedContainerColor.to_jni_value(),
		unfocusedContainerColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		errorContainerColor.to_jni_value(),
		cursorColor.to_jni_value(),
		errorCursorColor.to_jni_value(),
		selectionColors.to_jni_value(),
		focusedBorderColor.to_jni_value(),
		unfocusedBorderColor.to_jni_value(),
		disabledBorderColor.to_jni_value(),
		errorBorderColor.to_jni_value(),
		focusedLeadingIconColor.to_jni_value(),
		unfocusedLeadingIconColor.to_jni_value(),
		disabledLeadingIconColor.to_jni_value(),
		errorLeadingIconColor.to_jni_value(),
		focusedTrailingIconColor.to_jni_value(),
		unfocusedTrailingIconColor.to_jni_value(),
		disabledTrailingIconColor.to_jni_value(),
		errorTrailingIconColor.to_jni_value(),
		focusedLabelColor.to_jni_value(),
		unfocusedLabelColor.to_jni_value(),
		disabledLabelColor.to_jni_value(),
		errorLabelColor.to_jni_value(),
		focusedPlaceholderColor.to_jni_value(),
		unfocusedPlaceholderColor.to_jni_value(),
		disabledPlaceholderColor.to_jni_value(),
		errorPlaceholderColor.to_jni_value(),
		focusedSupportingTextColor.to_jni_value(),
		unfocusedSupportingTextColor.to_jni_value(),
		disabledSupportingTextColor.to_jni_value(),
		errorSupportingTextColor.to_jni_value(),
		focusedPrefixColor.to_jni_value(),
		unfocusedPrefixColor.to_jni_value(),
		disabledPrefixColor.to_jni_value(),
		errorPrefixColor.to_jni_value(),
		focusedSuffixColor.to_jni_value(),
		unfocusedSuffixColor.to_jni_value(),
		disabledSuffixColor.to_jni_value(),
		errorSuffixColor.to_jni_value(),
		defaultOutlinedTextFieldColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Tab.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Tab_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, text: Composable, icon: Composable, selectedContentColor: Color, unselectedContentColor: Color, interactionSource: MutableInteractionSource) -> jobject {
	let args = vec![
		selected.to_jni_value(),
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		text.to_jni_value(),
		icon.to_jni_value(),
		selectedContentColor.to_jni_value(),
		unselectedContentColor.to_jni_value(),
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Tab.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_LeadingIconTab_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean, onClick: fn() -> Unit, text: Composable, icon: Composable, modifier: Modifier, enabled: Boolean, selectedContentColor: Color, unselectedContentColor: Color, interactionSource: MutableInteractionSource) -> jobject {
	let args = vec![
		selected.to_jni_value(),
		onClick.to_jni_value(),
		text.to_jni_value(),
		icon.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		selectedContentColor.to_jni_value(),
		unselectedContentColor.to_jni_value(),
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Tab.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Tab_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, selectedContentColor: Color, unselectedContentColor: Color, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
	let args = vec![
		selected.to_jni_value(),
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		selectedContentColor.to_jni_value(),
		unselectedContentColor.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/FloatingActionButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_FloatingActionButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
	let args = vec![
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		shape.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		elevation.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/FloatingActionButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_SmallFloatingActionButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
	let args = vec![
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		shape.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		elevation.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/FloatingActionButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_LargeFloatingActionButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
	let args = vec![
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		shape.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		elevation.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/FloatingActionButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_ExtendedFloatingActionButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
	let args = vec![
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		shape.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		elevation.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/FloatingActionButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_ExtendedFloatingActionButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, text: Composable, icon: Composable, onClick: fn() -> Unit, modifier: Modifier, expanded: Boolean, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource) -> jobject {
	let args = vec![
		text.to_jni_value(),
		icon.to_jni_value(),
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		expanded.to_jni_value(),
		shape.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		elevation.to_jni_value(),
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/FloatingActionButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_elevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp) -> jobject {
	let args = vec![
		defaultElevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		defaultElevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		defaultElevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		defaultElevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/IconButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_IconButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, colors: IconButtonColors, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
	let args = vec![
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		colors.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/IconButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_IconToggleButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, checked: Boolean, onCheckedChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, colors: IconToggleButtonColors, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
	let args = vec![
		checked.to_jni_value(),
		onCheckedChange.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		colors.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/IconButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_FilledIconButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconButtonColors, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
	let args = vec![
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/IconButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_FilledTonalIconButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconButtonColors, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
	let args = vec![
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/IconButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_FilledIconToggleButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, checked: Boolean, onCheckedChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconToggleButtonColors, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
	let args = vec![
		checked.to_jni_value(),
		onCheckedChange.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/IconButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_FilledTonalIconToggleButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, checked: Boolean, onCheckedChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconToggleButtonColors, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
	let args = vec![
		checked.to_jni_value(),
		onCheckedChange.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/IconButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedIconButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconButtonColors, border: BorderStroke, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
	let args = vec![
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		border.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/IconButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedIconToggleButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, checked: Boolean, onCheckedChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconToggleButtonColors, border: BorderStroke, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
	let args = vec![
		checked.to_jni_value(),
		onCheckedChange.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		border.to_jni_value(),
		interactionSource.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/IconButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_iconButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/IconButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_iconButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, defaultIconButtonColors: Composable) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value(),
		defaultIconButtonColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/IconButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_iconToggleButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, checkedContainerColor: Color, checkedContentColor: Color, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, checkedContainerColor: Color, checkedContentColor: Color, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, checkedContainerColor: Color, checkedContentColor: Color, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, checkedContainerColor: Color, checkedContentColor: Color, enabled: Boolean, checked: Boolean): BorderStroke? {
        if (checked) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value(),
		checkedContainerColor.to_jni_value(),
		checkedContentColor.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value(),
		checkedContainerColor.to_jni_value(),
		checkedContentColor.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value(),
		checkedContainerColor.to_jni_value(),
		checkedContentColor.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value(),
		checkedContainerColor.to_jni_value(),
		checkedContentColor.to_jni_value(),
		enabled.to_jni_value(),
		checked.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/IconButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_outlinedIconButtonBorder_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, enabled: Boolean): BorderStroke {
        val color: Color) -> jobject {
	let args = vec![
		enabled.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/NavigationRail.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_NavigationRail_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, containerColor: Color, contentColor: Color, header: Composable, windowInsets: WindowInsets, content: Composable) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		header.to_jni_value(),
		windowInsets.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/NavigationRail.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_NavigationRailItem_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean, onClick: fn() -> Unit, icon: Composable, modifier: Modifier, enabled: Boolean, label: Composable, alwaysShowLabel: Boolean, colors: NavigationRailItemColors, interactionSource: MutableInteractionSource) -> jobject {
	let args = vec![
		selected.to_jni_value(),
		onClick.to_jni_value(),
		icon.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		label.to_jni_value(),
		alwaysShowLabel.to_jni_value(),
		colors.to_jni_value(),
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/NavigationRail.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, disabledIconColor: Color, disabledTextColor: Color, defaultNavigationRailItemColors: NavigationRailItemColors
        get() -> jobject {
	let args = vec![
		selectedIconColor.to_jni_value(),
		selectedTextColor.to_jni_value(),
		indicatorColor.to_jni_value(),
		unselectedIconColor.to_jni_value(),
		unselectedTextColor.to_jni_value(),
		disabledIconColor.to_jni_value(),
		disabledTextColor.to_jni_value(),
		defaultNavigationRailItemColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/NavigationRail.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, selectedIconColor: Color, selectedTextColor: Color, selectedIndicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, disabledIconColor: Color, disabledTextColor: Color) -> jobject {
	let args = vec![
		selectedIconColor.to_jni_value(),
		selectedTextColor.to_jni_value(),
		indicatorColor.to_jni_value(),
		unselectedIconColor.to_jni_value(),
		unselectedTextColor.to_jni_value(),
		selectedIconColor.to_jni_value(),
		selectedTextColor.to_jni_value(),
		selectedIndicatorColor.to_jni_value(),
		unselectedIconColor.to_jni_value(),
		unselectedTextColor.to_jni_value(),
		disabledIconColor.to_jni_value(),
		disabledTextColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/NavigationDrawer.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_rememberDrawerState_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, initialValue: DrawerValue, confirmStateChange: fn() -> Boolean) -> jobject {
	let args = vec![
		initialValue.to_jni_value(),
		confirmStateChange.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/NavigationDrawer.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_ModalNavigationDrawer_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, drawerContent: Composable, modifier: Modifier, drawerState: DrawerState, gesturesEnabled: Boolean, scrimColor: Color, content: Composable) -> jobject {
	let args = vec![
		drawerContent.to_jni_value(),
		modifier.to_jni_value(),
		drawerState.to_jni_value(),
		gesturesEnabled.to_jni_value(),
		scrimColor.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/NavigationDrawer.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_DismissibleNavigationDrawer_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, drawerContent: Composable, modifier: Modifier, drawerState: DrawerState, gesturesEnabled: Boolean, content: Composable) -> jobject {
	let args = vec![
		drawerContent.to_jni_value(),
		modifier.to_jni_value(),
		drawerState.to_jni_value(),
		gesturesEnabled.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/NavigationDrawer.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_PermanentNavigationDrawer_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, drawerContent: Composable, modifier: Modifier, content: Composable) -> jobject {
	let args = vec![
		drawerContent.to_jni_value(),
		modifier.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/NavigationDrawer.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_ModalDrawerSheet_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, drawerShape: Shape, drawerContainerColor: Color, drawerContentColor: Color, drawerTonalElevation: Dp, windowInsets: WindowInsets, content: Composable) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		drawerShape.to_jni_value(),
		drawerContainerColor.to_jni_value(),
		drawerContentColor.to_jni_value(),
		drawerTonalElevation.to_jni_value(),
		windowInsets.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/NavigationDrawer.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_DismissibleDrawerSheet_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, drawerShape: Shape, drawerContainerColor: Color, drawerContentColor: Color, drawerTonalElevation: Dp, windowInsets: WindowInsets, content: Composable) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		drawerShape.to_jni_value(),
		drawerContainerColor.to_jni_value(),
		drawerContentColor.to_jni_value(),
		drawerTonalElevation.to_jni_value(),
		windowInsets.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/NavigationDrawer.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_PermanentDrawerSheet_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, drawerShape: Shape, drawerContainerColor: Color, drawerContentColor: Color, drawerTonalElevation: Dp, windowInsets: WindowInsets, content: Composable) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		drawerShape.to_jni_value(),
		drawerContainerColor.to_jni_value(),
		drawerContentColor.to_jni_value(),
		drawerTonalElevation.to_jni_value(),
		windowInsets.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/NavigationDrawer.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_NavigationDrawerItem_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, label: Composable, selected: Boolean, onClick: fn() -> Unit, modifier: Modifier, icon: fn() -> Unit, badge: fn() -> Unit, shape: Shape, colors: NavigationDrawerItemColors, interactionSource: MutableInteractionSource) -> jobject {
	let args = vec![
		label.to_jni_value(),
		selected.to_jni_value(),
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		icon.to_jni_value(),
		badge.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/NavigationDrawer.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_iconColor_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean): State<Color>

    /**
     * Represents the text color for this item, selected: Boolean): State<Color>

    /**
     * Represents the badge color for this item, selected: Boolean): State<Color>

    /**
     * Represents the container color for this item, selected: Boolean): State<Color>
}

/** Defaults used in [NavigationDrawerItem], unselectedContainerColor: Color, selectedIconColor: Color, unselectedIconColor: Color, selectedTextColor: Color, unselectedTextColor: Color, selectedBadgeColor: Color, unselectedBadgeColor: Color, selectedIconColor: Color, unselectedIconColor: Color, selectedTextColor: Color, unselectedTextColor: Color, selectedContainerColor: Color, unselectedContainerColor: Color, selectedBadgeColor: Color, unselectedBadgeColor: Composable, b: Float, pos: Float), open: Boolean, onClose: fn() -> Unit, fraction: fn() -> Float, color: Color) -> jobject {
	let args = vec![
		selected.to_jni_value(),
		selected.to_jni_value(),
		selected.to_jni_value(),
		selected.to_jni_value(),
		unselectedContainerColor.to_jni_value(),
		selectedIconColor.to_jni_value(),
		unselectedIconColor.to_jni_value(),
		selectedTextColor.to_jni_value(),
		unselectedTextColor.to_jni_value(),
		selectedBadgeColor.to_jni_value(),
		unselectedBadgeColor.to_jni_value(),
		selectedIconColor.to_jni_value(),
		unselectedIconColor.to_jni_value(),
		selectedTextColor.to_jni_value(),
		unselectedTextColor.to_jni_value(),
		selectedContainerColor.to_jni_value(),
		unselectedContainerColor.to_jni_value(),
		selectedBadgeColor.to_jni_value(),
		unselectedBadgeColor.to_jni_value(),
		b.to_jni_value(),
		pos.to_jni_value(),
		open.to_jni_value(),
		onClose.to_jni_value(),
		fraction.to_jni_value(),
		color.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TabRow.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_PrimaryTabRow_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, indicator: Composable, divider: Composable, tabs: Composable) -> jobject {
	let args = vec![
		selectedTabIndex.to_jni_value(),
		modifier.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		indicator.to_jni_value(),
		divider.to_jni_value(),
		tabs.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TabRow.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_SecondaryTabRow_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, indicator: Composable, divider: Composable, tabs: Composable) -> jobject {
	let args = vec![
		selectedTabIndex.to_jni_value(),
		modifier.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		indicator.to_jni_value(),
		divider.to_jni_value(),
		tabs.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TabRow.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_TabRow_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, indicator: Composable) -> jobject {
	let args = vec![
		selectedTabIndex.to_jni_value(),
		modifier.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		indicator.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TabRow.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_PrimaryScrollableTabRow_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedTabIndex: Int, modifier: Modifier, scrollState: ScrollState, containerColor: Color, contentColor: Color, edgePadding: Dp, indicator: Composable) -> jobject {
	let args = vec![
		selectedTabIndex.to_jni_value(),
		modifier.to_jni_value(),
		scrollState.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		edgePadding.to_jni_value(),
		indicator.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TabRow.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_SecondaryScrollableTabRow_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedTabIndex: Int, modifier: Modifier, scrollState: ScrollState, containerColor: Color, contentColor: Color, edgePadding: Dp, indicator: Composable, divider: Composable, tabs: Composable) -> jobject {
	let args = vec![
		selectedTabIndex.to_jni_value(),
		modifier.to_jni_value(),
		scrollState.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		edgePadding.to_jni_value(),
		indicator.to_jni_value(),
		divider.to_jni_value(),
		tabs.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TabRow.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_ScrollableTabRow_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, edgePadding: Dp, indicator: Composable, divider: Composable, tabs: Composable) -> jobject {
	let args = vec![
		selectedTabIndex.to_jni_value(),
		modifier.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		edgePadding.to_jni_value(),
		indicator.to_jni_value(),
		divider.to_jni_value(),
		tabs.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TabRow.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_PrimaryIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, width: Dp, height: Dp, color: Color, shape: Shape) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		width.to_jni_value(),
		height.to_jni_value(),
		color.to_jni_value(),
		shape.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/TabRow.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_SecondaryIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, height: Dp, color: Color) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		height.to_jni_value(),
		color.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/SegmentedButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, activeContainerColor: Color, activeContentColor: Color, activeBorderColor: Color, inactiveContainerColor: Color, inactiveContentColor: Color, inactiveBorderColor: Color, disabledActiveContainerColor: Color, disabledActiveContentColor: Color, disabledActiveBorderColor: Color, disabledInactiveContainerColor: Color, disabledInactiveContentColor: Color, disabledInactiveBorderColor: Color, defaultSegmentedButtonColors: SegmentedButtonColors
        get() -> jobject {
	let args = vec![
		activeContainerColor.to_jni_value(),
		activeContentColor.to_jni_value(),
		activeBorderColor.to_jni_value(),
		inactiveContainerColor.to_jni_value(),
		inactiveContentColor.to_jni_value(),
		inactiveBorderColor.to_jni_value(),
		disabledActiveContainerColor.to_jni_value(),
		disabledActiveContentColor.to_jni_value(),
		disabledActiveBorderColor.to_jni_value(),
		disabledInactiveContainerColor.to_jni_value(),
		disabledInactiveContentColor.to_jni_value(),
		disabledInactiveBorderColor.to_jni_value(),
		defaultSegmentedButtonColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/SegmentedButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_ActiveIcon_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/SegmentedButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Icon_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, active: Boolean, activeContent: Composable, inactiveContent: fn() -> Unit) -> jobject {
	let args = vec![
		active.to_jni_value(),
		activeContent.to_jni_value(),
		inactiveContent.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Icon.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Icon_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, imageVector: ImageVector, contentDescription: String, modifier: Modifier, tint: Color) -> jobject {
	let args = vec![
		imageVector.to_jni_value(),
		contentDescription.to_jni_value(),
		modifier.to_jni_value(),
		tint.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Icon.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Icon_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, bitmap: ImageBitmap, contentDescription: String, modifier: Modifier, tint: Color) -> jobject {
	let args = vec![
		bitmap.to_jni_value(),
		contentDescription.to_jni_value(),
		modifier.to_jni_value(),
		tint.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Icon.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Icon_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, painter: Painter, contentDescription: String, modifier: Modifier, tint: Color) -> jobject {
	let args = vec![
		painter.to_jni_value(),
		contentDescription.to_jni_value(),
		modifier.to_jni_value(),
		tint.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/AppBar.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_TopAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, title: Composable, modifier: Modifier, navigationIcon: Composable, actions: Composable, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior) -> jobject {
	let args = vec![
		title.to_jni_value(),
		modifier.to_jni_value(),
		navigationIcon.to_jni_value(),
		actions.to_jni_value(),
		windowInsets.to_jni_value(),
		colors.to_jni_value(),
		scrollBehavior.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/AppBar.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_SmallTopAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, title: Composable, modifier: Modifier, navigationIcon: Composable, actions: Composable, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior, https: //m3, like: * @sample androidx, title: Composable, modifier: Modifier, navigationIcon: Composable, actions: Composable, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior) -> jobject {
	let args = vec![
		title.to_jni_value(),
		modifier.to_jni_value(),
		navigationIcon.to_jni_value(),
		actions.to_jni_value(),
		windowInsets.to_jni_value(),
		colors.to_jni_value(),
		scrollBehavior.to_jni_value(),
		https.to_jni_value(),
		like.to_jni_value(),
		title.to_jni_value(),
		modifier.to_jni_value(),
		navigationIcon.to_jni_value(),
		actions.to_jni_value(),
		windowInsets.to_jni_value(),
		colors.to_jni_value(),
		scrollBehavior.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/AppBar.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_MediumTopAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, title: Composable, modifier: Modifier, navigationIcon: Composable, actions: Composable, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior) -> jobject {
	let args = vec![
		title.to_jni_value(),
		modifier.to_jni_value(),
		navigationIcon.to_jni_value(),
		actions.to_jni_value(),
		windowInsets.to_jni_value(),
		colors.to_jni_value(),
		scrollBehavior.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/AppBar.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_LargeTopAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, title: Composable, modifier: Modifier, navigationIcon: Composable, actions: Composable, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior) -> jobject {
	let args = vec![
		title.to_jni_value(),
		modifier.to_jni_value(),
		navigationIcon.to_jni_value(),
		actions.to_jni_value(),
		windowInsets.to_jni_value(),
		colors.to_jni_value(),
		scrollBehavior.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/AppBar.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_BottomAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, actions: Composable, modifier: Modifier, floatingActionButton: Composable, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, https: //m3, also: [Surface], modifier: Modifier, floatingActionButton: Composable, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, scrollBehavior: BottomAppBarScrollBehavior) -> jobject {
	let args = vec![
		actions.to_jni_value(),
		modifier.to_jni_value(),
		floatingActionButton.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		tonalElevation.to_jni_value(),
		contentPadding.to_jni_value(),
		windowInsets.to_jni_value(),
		https.to_jni_value(),
		also.to_jni_value(),
		modifier.to_jni_value(),
		floatingActionButton.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		tonalElevation.to_jni_value(),
		contentPadding.to_jni_value(),
		windowInsets.to_jni_value(),
		scrollBehavior.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/AppBar.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_BottomAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, content: Composable, https: //m3, also: [Surface], modifier: Modifier, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, scrollBehavior: BottomAppBarScrollBehavior, content: Composable) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		tonalElevation.to_jni_value(),
		contentPadding.to_jni_value(),
		windowInsets.to_jni_value(),
		content.to_jni_value(),
		https.to_jni_value(),
		also.to_jni_value(),
		modifier.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		tonalElevation.to_jni_value(),
		contentPadding.to_jni_value(),
		windowInsets.to_jni_value(),
		scrollBehavior.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/AppBar.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_topAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color, defaultTopAppBarColors: TopAppBarColors
        get() -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		scrolledContainerColor.to_jni_value(),
		navigationIconContentColor.to_jni_value(),
		titleContentColor.to_jni_value(),
		actionIconContentColor.to_jni_value(),
		defaultTopAppBarColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/AppBar.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_smallTopAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color, windowInsets: Composable, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color, defaultCenterAlignedTopAppBarColors: TopAppBarColors
        get() -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		scrolledContainerColor.to_jni_value(),
		navigationIconContentColor.to_jni_value(),
		titleContentColor.to_jni_value(),
		actionIconContentColor.to_jni_value(),
		windowInsets.to_jni_value(),
		scrolledContainerColor.to_jni_value(),
		navigationIconContentColor.to_jni_value(),
		titleContentColor.to_jni_value(),
		actionIconContentColor.to_jni_value(),
		defaultCenterAlignedTopAppBarColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/AppBar.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_mediumTopAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color, defaultMediumTopAppBarColors: TopAppBarColors
        get() -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		scrolledContainerColor.to_jni_value(),
		navigationIconContentColor.to_jni_value(),
		titleContentColor.to_jni_value(),
		actionIconContentColor.to_jni_value(),
		defaultMediumTopAppBarColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/AppBar.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_largeTopAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color, defaultLargeTopAppBarColors: TopAppBarColors
        get() -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		scrolledContainerColor.to_jni_value(),
		navigationIconContentColor.to_jni_value(),
		titleContentColor.to_jni_value(),
		actionIconContentColor.to_jni_value(),
		defaultLargeTopAppBarColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/AppBar.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_pinnedScrollBehavior_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: TopAppBarState, canScroll: fn() -> Boolean, state: TopAppBarState, canScroll: fn() -> Boolean, snapAnimationSpec: AnimationSpec<Float>, flingAnimationSpec: DecayAnimationSpec<Float>, state: TopAppBarState, canScroll: fn() -> Boolean, snapAnimationSpec: AnimationSpec<Float>, flingAnimationSpec: DecayAnimationSpec<Float>, initialHeightOffsetLimit: Float, initialHeightOffset: Float, initialContentOffset: Float) -> jobject {
	let args = vec![
		state.to_jni_value(),
		canScroll.to_jni_value(),
		state.to_jni_value(),
		canScroll.to_jni_value(),
		snapAnimationSpec.to_jni_value(),
		flingAnimationSpec.to_jni_value(),
		state.to_jni_value(),
		canScroll.to_jni_value(),
		snapAnimationSpec.to_jni_value(),
		flingAnimationSpec.to_jni_value(),
		initialHeightOffsetLimit.to_jni_value(),
		initialHeightOffset.to_jni_value(),
		initialContentOffset.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/AppBar.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_exitAlwaysScrollBehavior_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: BottomAppBarState, canScroll: fn() -> Boolean, snapAnimationSpec: AnimationSpec<Float>, flingAnimationSpec: DecayAnimationSpec<Float>, initialHeightOffsetLimit: Float, initialHeightOffset: Float, initialContentOffset: Float) -> jobject {
	let args = vec![
		state.to_jni_value(),
		canScroll.to_jni_value(),
		snapAnimationSpec.to_jni_value(),
		flingAnimationSpec.to_jni_value(),
		initialHeightOffsetLimit.to_jni_value(),
		initialHeightOffset.to_jni_value(),
		initialContentOffset.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/OutlinedTextField.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedTextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: String, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, prefix: Composable, suffix: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
	let args = vec![
		value.to_jni_value(),
		onValueChange.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		readOnly.to_jni_value(),
		textStyle.to_jni_value(),
		label.to_jni_value(),
		placeholder.to_jni_value(),
		leadingIcon.to_jni_value(),
		trailingIcon.to_jni_value(),
		prefix.to_jni_value(),
		suffix.to_jni_value(),
		supportingText.to_jni_value(),
		isError.to_jni_value(),
		visualTransformation.to_jni_value(),
		keyboardOptions.to_jni_value(),
		keyboardActions.to_jni_value(),
		singleLine.to_jni_value(),
		maxLines.to_jni_value(),
		minLines.to_jni_value(),
		interactionSource.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/OutlinedTextField.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedTextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: TextFieldValue, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, prefix: Composable, suffix: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
	let args = vec![
		value.to_jni_value(),
		onValueChange.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		readOnly.to_jni_value(),
		textStyle.to_jni_value(),
		label.to_jni_value(),
		placeholder.to_jni_value(),
		leadingIcon.to_jni_value(),
		trailingIcon.to_jni_value(),
		prefix.to_jni_value(),
		suffix.to_jni_value(),
		supportingText.to_jni_value(),
		isError.to_jni_value(),
		visualTransformation.to_jni_value(),
		keyboardOptions.to_jni_value(),
		keyboardActions.to_jni_value(),
		singleLine.to_jni_value(),
		maxLines.to_jni_value(),
		minLines.to_jni_value(),
		interactionSource.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/OutlinedTextField.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedTextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: String, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
	let args = vec![
		value.to_jni_value(),
		onValueChange.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		readOnly.to_jni_value(),
		textStyle.to_jni_value(),
		label.to_jni_value(),
		placeholder.to_jni_value(),
		leadingIcon.to_jni_value(),
		trailingIcon.to_jni_value(),
		supportingText.to_jni_value(),
		isError.to_jni_value(),
		visualTransformation.to_jni_value(),
		keyboardOptions.to_jni_value(),
		keyboardActions.to_jni_value(),
		singleLine.to_jni_value(),
		maxLines.to_jni_value(),
		minLines.to_jni_value(),
		interactionSource.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/OutlinedTextField.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedTextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: TextFieldValue, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
	let args = vec![
		value.to_jni_value(),
		onValueChange.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		readOnly.to_jni_value(),
		textStyle.to_jni_value(),
		label.to_jni_value(),
		placeholder.to_jni_value(),
		leadingIcon.to_jni_value(),
		trailingIcon.to_jni_value(),
		supportingText.to_jni_value(),
		isError.to_jni_value(),
		visualTransformation.to_jni_value(),
		keyboardOptions.to_jni_value(),
		keyboardActions.to_jni_value(),
		singleLine.to_jni_value(),
		maxLines.to_jni_value(),
		minLines.to_jni_value(),
		interactionSource.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Snackbar.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Snackbar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, action: Composable, dismissAction: Composable, actionOnNewLine: Boolean, shape: Shape, containerColor: Color, contentColor: Color, actionContentColor: Color, dismissActionContentColor: Color, content: Composable) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		action.to_jni_value(),
		dismissAction.to_jni_value(),
		actionOnNewLine.to_jni_value(),
		shape.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		actionContentColor.to_jni_value(),
		dismissActionContentColor.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Snackbar.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Snackbar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, snackbarData: SnackbarData, modifier: Modifier, actionOnNewLine: Boolean, shape: Shape, containerColor: Color, contentColor: Color, actionColor: Color, actionContentColor: Color, dismissActionContentColor: Color) -> jobject {
	let args = vec![
		snackbarData.to_jni_value(),
		modifier.to_jni_value(),
		actionOnNewLine.to_jni_value(),
		shape.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		actionColor.to_jni_value(),
		actionContentColor.to_jni_value(),
		dismissActionContentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ListItem.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_ListItem_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, headlineContent: Composable, modifier: Modifier, overlineContent: Composable, supportingContent: Composable, leadingContent: Composable, trailingContent: Composable, colors: ListItemColors, tonalElevation: Dp, shadowElevation: Dp) -> jobject {
	let args = vec![
		headlineContent.to_jni_value(),
		modifier.to_jni_value(),
		overlineContent.to_jni_value(),
		supportingContent.to_jni_value(),
		leadingContent.to_jni_value(),
		trailingContent.to_jni_value(),
		colors.to_jni_value(),
		tonalElevation.to_jni_value(),
		shadowElevation.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ListItem.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, headlineColor: Color, leadingIconColor: Color, overlineColor: Color, supportingColor: Color, trailingIconColor: Color, disabledHeadlineColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, containerColor: Color, headlineColor: Color, leadingIconColor: Color, overlineColor: Color, supportingTextColor: Color, trailingIconColor: Color, disabledHeadlineColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		headlineColor.to_jni_value(),
		leadingIconColor.to_jni_value(),
		overlineColor.to_jni_value(),
		supportingColor.to_jni_value(),
		trailingIconColor.to_jni_value(),
		disabledHeadlineColor.to_jni_value(),
		disabledLeadingIconColor.to_jni_value(),
		disabledTrailingIconColor.to_jni_value(),
		containerColor.to_jni_value(),
		headlineColor.to_jni_value(),
		leadingIconColor.to_jni_value(),
		overlineColor.to_jni_value(),
		supportingTextColor.to_jni_value(),
		trailingIconColor.to_jni_value(),
		disabledHeadlineColor.to_jni_value(),
		disabledLeadingIconColor.to_jni_value(),
		disabledTrailingIconColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/SheetDefaults.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_DragHandle_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, width: Dp, height: Dp, shape: Shape, color: Color) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		width.to_jni_value(),
		height.to_jni_value(),
		shape.to_jni_value(),
		color.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Switch.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, checkedThumbColor: Color, checkedTrackColor: Color, checkedBorderColor: Color, checkedIconColor: Color, uncheckedThumbColor: Color, uncheckedTrackColor: Color, uncheckedBorderColor: Color, uncheckedIconColor: Color, disabledCheckedThumbColor: Color, disabledCheckedTrackColor: Color, disabledCheckedBorderColor: Color, disabledCheckedIconColor: Color, disabledUncheckedThumbColor: Color, disabledUncheckedTrackColor: Color, disabledUncheckedBorderColor: Color, disabledUncheckedIconColor: Color, defaultSwitchColors: SwitchColors
        get() -> jobject {
	let args = vec![
		checkedThumbColor.to_jni_value(),
		checkedTrackColor.to_jni_value(),
		checkedBorderColor.to_jni_value(),
		checkedIconColor.to_jni_value(),
		uncheckedThumbColor.to_jni_value(),
		uncheckedTrackColor.to_jni_value(),
		uncheckedBorderColor.to_jni_value(),
		uncheckedIconColor.to_jni_value(),
		disabledCheckedThumbColor.to_jni_value(),
		disabledCheckedTrackColor.to_jni_value(),
		disabledCheckedBorderColor.to_jni_value(),
		disabledCheckedIconColor.to_jni_value(),
		disabledUncheckedThumbColor.to_jni_value(),
		disabledUncheckedTrackColor.to_jni_value(),
		disabledUncheckedBorderColor.to_jni_value(),
		disabledUncheckedIconColor.to_jni_value(),
		defaultSwitchColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/RadioButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_RadioButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, colors: RadioButtonColors, interactionSource: MutableInteractionSource) -> jobject {
	let args = vec![
		selected.to_jni_value(),
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		colors.to_jni_value(),
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/RadioButton.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedColor: Color, unselectedColor: Color, disabledSelectedColor: Color, disabledUnselectedColor: Color, defaultRadioButtonColors: RadioButtonColors
        get() -> jobject {
	let args = vec![
		selectedColor.to_jni_value(),
		unselectedColor.to_jni_value(),
		disabledSelectedColor.to_jni_value(),
		disabledUnselectedColor.to_jni_value(),
		defaultRadioButtonColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Badge.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_BadgedBox_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, badge: Composable, modifier: Modifier, content: Composable) -> jobject {
	let args = vec![
		badge.to_jni_value(),
		modifier.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Badge.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Badge_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, containerColor: Color, contentColor: Color, content: Composable) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Tooltip.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_richTooltipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, titleContentColor: Color, actionContentColor: Color, defaultRichTooltipColors: RichTooltipColors
        get() -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		titleContentColor.to_jni_value(),
		actionContentColor.to_jni_value(),
		defaultRichTooltipColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Tooltip.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_rememberPlainTooltipPositionProvider_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, spacingBetweenTooltipAndAnchor: Dp) -> jobject {
	let args = vec![
		spacingBetweenTooltipAndAnchor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Tooltip.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_rememberRichTooltipPositionProvider_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, spacingBetweenTooltipAndAnchor: Dp) -> jobject {
	let args = vec![
		spacingBetweenTooltipAndAnchor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Chip.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_AssistChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource, https: //m3, https: //developer, AssistChip: * @sample androidx, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource, https: //m3, https: //developer, icon: * @sample androidx, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource, https: //m3, https: //developer, icon: * @sample androidx, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource, https: //m3, https: //developer, icon: * @sample androidx, selected: Boolean, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: SelectableChipColors, elevation: SelectableChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource, https: //m3, https: //developer, icon: * @sample androidx, selected: Boolean, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: SelectableChipColors, elevation: SelectableChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource, https: //m3, https: //developer, icon: * @sample androidx, selected: Boolean, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, avatar: Composable, trailingIcon: Composable, shape: Shape, colors: SelectableChipColors, elevation: SelectableChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource) -> jobject {
	let args = vec![
		onClick.to_jni_value(),
		label.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		leadingIcon.to_jni_value(),
		trailingIcon.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		interactionSource.to_jni_value(),
		https.to_jni_value(),
		https.to_jni_value(),
		AssistChip.to_jni_value(),
		onClick.to_jni_value(),
		label.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		leadingIcon.to_jni_value(),
		trailingIcon.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		interactionSource.to_jni_value(),
		https.to_jni_value(),
		https.to_jni_value(),
		icon.to_jni_value(),
		onClick.to_jni_value(),
		label.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		leadingIcon.to_jni_value(),
		trailingIcon.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		interactionSource.to_jni_value(),
		https.to_jni_value(),
		https.to_jni_value(),
		icon.to_jni_value(),
		onClick.to_jni_value(),
		label.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		leadingIcon.to_jni_value(),
		trailingIcon.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		interactionSource.to_jni_value(),
		https.to_jni_value(),
		https.to_jni_value(),
		icon.to_jni_value(),
		selected.to_jni_value(),
		onClick.to_jni_value(),
		label.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		leadingIcon.to_jni_value(),
		trailingIcon.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		interactionSource.to_jni_value(),
		https.to_jni_value(),
		https.to_jni_value(),
		icon.to_jni_value(),
		selected.to_jni_value(),
		onClick.to_jni_value(),
		label.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		leadingIcon.to_jni_value(),
		trailingIcon.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		interactionSource.to_jni_value(),
		https.to_jni_value(),
		https.to_jni_value(),
		icon.to_jni_value(),
		selected.to_jni_value(),
		onClick.to_jni_value(),
		label.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		leadingIcon.to_jni_value(),
		avatar.to_jni_value(),
		trailingIcon.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Chip.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_SuggestionChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, icon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource, https: //m3, https: //developer, icon: * @sample androidx, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, icon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource, https: //m3, https: //developer, icon: * @sample androidx, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, icon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource, https: //m3, https: //developer, icon: * @sample androidx, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, icon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource, containerColor: Color, labelColor: Color, leadingIconContentColor: Color, trailingIconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconContentColor: Color, disabledTrailingIconContentColor: Color, defaultAssistChipColors: ChipColors
        get() -> jobject {
	let args = vec![
		onClick.to_jni_value(),
		label.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		icon.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		interactionSource.to_jni_value(),
		https.to_jni_value(),
		https.to_jni_value(),
		icon.to_jni_value(),
		onClick.to_jni_value(),
		label.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		icon.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		interactionSource.to_jni_value(),
		https.to_jni_value(),
		https.to_jni_value(),
		icon.to_jni_value(),
		onClick.to_jni_value(),
		label.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		icon.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		interactionSource.to_jni_value(),
		https.to_jni_value(),
		https.to_jni_value(),
		icon.to_jni_value(),
		onClick.to_jni_value(),
		label.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		icon.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		interactionSource.to_jni_value(),
		containerColor.to_jni_value(),
		labelColor.to_jni_value(),
		leadingIconContentColor.to_jni_value(),
		trailingIconContentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledLabelColor.to_jni_value(),
		disabledLeadingIconContentColor.to_jni_value(),
		disabledTrailingIconContentColor.to_jni_value(),
		defaultAssistChipColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Chip.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_assistChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, enabled: Boolean, borderColor: Color, disabledBorderColor: Color, borderWidth: Dp, borderColor: Color, disabledBorderColor: Color, borderWidth: Dp, containerColor: Color, labelColor: Color, leadingIconContentColor: Color, trailingIconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconContentColor: Color, disabledTrailingIconContentColor: Color, defaultElevatedAssistChipColors: ChipColors
        get() -> jobject {
	let args = vec![
		elevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value(),
		enabled.to_jni_value(),
		borderColor.to_jni_value(),
		disabledBorderColor.to_jni_value(),
		borderWidth.to_jni_value(),
		borderColor.to_jni_value(),
		disabledBorderColor.to_jni_value(),
		borderWidth.to_jni_value(),
		containerColor.to_jni_value(),
		labelColor.to_jni_value(),
		leadingIconContentColor.to_jni_value(),
		trailingIconContentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledLabelColor.to_jni_value(),
		disabledLeadingIconContentColor.to_jni_value(),
		disabledTrailingIconContentColor.to_jni_value(),
		defaultElevatedAssistChipColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Chip.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedAssistChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, shape: Composable, labelColor: Color, iconColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, selectedContainerColor: Color, disabledSelectedContainerColor: Color, selectedLabelColor: Color, selectedLeadingIconColor: Color, selectedTrailingIconColor: Color, defaultFilterChipColors: SelectableChipColors
        get() -> jobject {
	let args = vec![
		elevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value(),
		shape.to_jni_value(),
		labelColor.to_jni_value(),
		iconColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledLabelColor.to_jni_value(),
		disabledLeadingIconColor.to_jni_value(),
		disabledTrailingIconColor.to_jni_value(),
		selectedContainerColor.to_jni_value(),
		disabledSelectedContainerColor.to_jni_value(),
		selectedLabelColor.to_jni_value(),
		selectedLeadingIconColor.to_jni_value(),
		selectedTrailingIconColor.to_jni_value(),
		defaultFilterChipColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Chip.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_filterChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, enabled: Boolean, selected: Boolean, borderColor: Color, selectedBorderColor: Color, disabledBorderColor: Color, disabledSelectedBorderColor: Color, borderWidth: Dp, selectedBorderWidth: Dp) -> jobject {
	let args = vec![
		elevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value(),
		enabled.to_jni_value(),
		selected.to_jni_value(),
		borderColor.to_jni_value(),
		selectedBorderColor.to_jni_value(),
		disabledBorderColor.to_jni_value(),
		disabledSelectedBorderColor.to_jni_value(),
		borderWidth.to_jni_value(),
		selectedBorderWidth.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Chip.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedFilterChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, labelColor: Color, iconColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, selectedContainerColor: Color, disabledSelectedContainerColor: Color, selectedLabelColor: Color, selectedLeadingIconColor: Color, selectedTrailingIconColor: Color, defaultElevatedFilterChipColors: SelectableChipColors
        get() -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		labelColor.to_jni_value(),
		iconColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledLabelColor.to_jni_value(),
		disabledLeadingIconColor.to_jni_value(),
		disabledTrailingIconColor.to_jni_value(),
		selectedContainerColor.to_jni_value(),
		disabledSelectedContainerColor.to_jni_value(),
		selectedLabelColor.to_jni_value(),
		selectedLeadingIconColor.to_jni_value(),
		selectedTrailingIconColor.to_jni_value(),
		defaultElevatedFilterChipColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Chip.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedFilterChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, shape: Composable, containerColor: Color, labelColor: Color, leadingIconColor: Color, trailingIconColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, selectedContainerColor: Color, disabledSelectedContainerColor: Color, selectedLabelColor: Color, selectedLeadingIconColor: Color, selectedTrailingIconColor: Color, defaultInputChipColors: SelectableChipColors
        get() -> jobject {
	let args = vec![
		elevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value(),
		shape.to_jni_value(),
		containerColor.to_jni_value(),
		labelColor.to_jni_value(),
		leadingIconColor.to_jni_value(),
		trailingIconColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledLabelColor.to_jni_value(),
		disabledLeadingIconColor.to_jni_value(),
		disabledTrailingIconColor.to_jni_value(),
		selectedContainerColor.to_jni_value(),
		disabledSelectedContainerColor.to_jni_value(),
		selectedLabelColor.to_jni_value(),
		selectedLeadingIconColor.to_jni_value(),
		selectedTrailingIconColor.to_jni_value(),
		defaultInputChipColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Chip.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_inputChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, enabled: Boolean, selected: Boolean, borderColor: Color, selectedBorderColor: Color, disabledBorderColor: Color, disabledSelectedBorderColor: Color, borderWidth: Dp, selectedBorderWidth: Dp) -> jobject {
	let args = vec![
		elevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value(),
		enabled.to_jni_value(),
		selected.to_jni_value(),
		borderColor.to_jni_value(),
		selectedBorderColor.to_jni_value(),
		disabledBorderColor.to_jni_value(),
		disabledSelectedBorderColor.to_jni_value(),
		borderWidth.to_jni_value(),
		selectedBorderWidth.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Chip.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_suggestionChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, labelColor: Color, iconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledIconContentColor: Color, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, enabled: Boolean, borderColor: Color, disabledBorderColor: Color, borderWidth: Dp, borderColor: Color, disabledBorderColor: Color, borderWidth: Dp, containerColor: Color, labelColor: Color, iconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledIconContentColor: Color, defaultElevatedSuggestionChipColors: ChipColors
        get() -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		labelColor.to_jni_value(),
		iconContentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledLabelColor.to_jni_value(),
		disabledIconContentColor.to_jni_value(),
		elevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value(),
		enabled.to_jni_value(),
		borderColor.to_jni_value(),
		disabledBorderColor.to_jni_value(),
		borderWidth.to_jni_value(),
		borderColor.to_jni_value(),
		disabledBorderColor.to_jni_value(),
		borderWidth.to_jni_value(),
		containerColor.to_jni_value(),
		labelColor.to_jni_value(),
		iconContentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledLabelColor.to_jni_value(),
		disabledIconContentColor.to_jni_value(),
		defaultElevatedSuggestionChipColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Chip.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedSuggestionChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, shape: Composable, onClick: fn() -> Unit, enabled: Boolean, label: Composable, labelTextStyle: TextStyle, labelColor: Color, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, minHeight: Dp, paddingValues: PaddingValues, interactionSource: MutableInteractionSource) -> jobject {
	let args = vec![
		elevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value(),
		shape.to_jni_value(),
		onClick.to_jni_value(),
		enabled.to_jni_value(),
		label.to_jni_value(),
		labelTextStyle.to_jni_value(),
		labelColor.to_jni_value(),
		leadingIcon.to_jni_value(),
		trailingIcon.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		border.to_jni_value(),
		minHeight.to_jni_value(),
		paddingValues.to_jni_value(),
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/DateRangePicker.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_DateRangePicker_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: DateRangePickerState, modifier: Modifier, dateFormatter: DatePickerFormatter, title: fn() -> Unit, headline: fn() -> Unit, showModeToggle: Boolean, colors: DatePickerColors) -> jobject {
	let args = vec![
		state.to_jni_value(),
		modifier.to_jni_value(),
		dateFormatter.to_jni_value(),
		title.to_jni_value(),
		headline.to_jni_value(),
		showModeToggle.to_jni_value(),
		colors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/DateRangePicker.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_DateRangePickerTitle_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, displayMode: DisplayMode, modifier: Modifier) -> jobject {
	let args = vec![
		displayMode.to_jni_value(),
		modifier.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/DateRangePicker.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_DateRangePickerHeadline_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedStartDateMillis: Long, selectedEndDateMillis: Long, displayMode: DisplayMode, dateFormatter: DatePickerFormatter, modifier: Modifier) -> jobject {
	let args = vec![
		selectedStartDateMillis.to_jni_value(),
		selectedEndDateMillis.to_jni_value(),
		displayMode.to_jni_value(),
		dateFormatter.to_jni_value(),
		modifier.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/SnackbarHost.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_SnackbarHost_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, hostState: SnackbarHostState, modifier: Modifier, snackbar: Composable) -> jobject {
	let args = vec![
		hostState.to_jni_value(),
		modifier.to_jni_value(),
		snackbar.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ProgressIndicator.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_LinearProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, progress: fn() -> Float, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap) -> jobject {
	let args = vec![
		progress.to_jni_value(),
		modifier.to_jni_value(),
		color.to_jni_value(),
		trackColor.to_jni_value(),
		strokeCap.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ProgressIndicator.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_LinearProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, progress: fn() -> Float, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap, gapSize: Dp, drawStopIndicator: (DrawScope) -> jobject {
	let args = vec![
		progress.to_jni_value(),
		modifier.to_jni_value(),
		color.to_jni_value(),
		trackColor.to_jni_value(),
		strokeCap.to_jni_value(),
		gapSize.to_jni_value(),
		drawStopIndicator.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ProgressIndicator.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_LinearProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		color.to_jni_value(),
		trackColor.to_jni_value(),
		strokeCap.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ProgressIndicator.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_LinearProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap, gapSize: Dp) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		color.to_jni_value(),
		trackColor.to_jni_value(),
		strokeCap.to_jni_value(),
		gapSize.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ProgressIndicator.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_LinearProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, progress: Float, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap, progress: Float, modifier: Modifier, color: Color, trackColor: Color, modifier: Modifier, color: Color, trackColor: Color, startFraction: Float, endFraction: Float, color: Color, strokeWidth: Float, strokeCap: StrokeCap) -> jobject {
	let args = vec![
		progress.to_jni_value(),
		modifier.to_jni_value(),
		color.to_jni_value(),
		trackColor.to_jni_value(),
		strokeCap.to_jni_value(),
		progress.to_jni_value(),
		modifier.to_jni_value(),
		color.to_jni_value(),
		trackColor.to_jni_value(),
		modifier.to_jni_value(),
		color.to_jni_value(),
		trackColor.to_jni_value(),
		startFraction.to_jni_value(),
		endFraction.to_jni_value(),
		color.to_jni_value(),
		strokeWidth.to_jni_value(),
		strokeCap.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ProgressIndicator.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_CircularProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, progress: fn() -> Float, modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap) -> jobject {
	let args = vec![
		progress.to_jni_value(),
		modifier.to_jni_value(),
		color.to_jni_value(),
		strokeWidth.to_jni_value(),
		trackColor.to_jni_value(),
		strokeCap.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ProgressIndicator.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_CircularProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, progress: fn() -> Float, modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap, gapSize: Dp) -> jobject {
	let args = vec![
		progress.to_jni_value(),
		modifier.to_jni_value(),
		color.to_jni_value(),
		strokeWidth.to_jni_value(),
		trackColor.to_jni_value(),
		strokeCap.to_jni_value(),
		gapSize.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ProgressIndicator.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_CircularProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		color.to_jni_value(),
		strokeWidth.to_jni_value(),
		trackColor.to_jni_value(),
		strokeCap.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ProgressIndicator.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_CircularProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, progress: Float, modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap, progress: Float, modifier: Modifier, color: Color, strokeWidth: Dp, modifier: Modifier, color: Color, strokeWidth: Dp, startAngle: Float, sweep: Float, color: Color, stroke: Stroke) -> jobject {
	let args = vec![
		progress.to_jni_value(),
		modifier.to_jni_value(),
		color.to_jni_value(),
		strokeWidth.to_jni_value(),
		trackColor.to_jni_value(),
		strokeCap.to_jni_value(),
		progress.to_jni_value(),
		modifier.to_jni_value(),
		color.to_jni_value(),
		strokeWidth.to_jni_value(),
		modifier.to_jni_value(),
		color.to_jni_value(),
		strokeWidth.to_jni_value(),
		startAngle.to_jni_value(),
		sweep.to_jni_value(),
		color.to_jni_value(),
		stroke.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Divider.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_HorizontalDivider_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, thickness: Dp, color: Color) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		thickness.to_jni_value(),
		color.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Divider.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_VerticalDivider_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, thickness: Dp, color: Color) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		thickness.to_jni_value(),
		color.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Divider.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Divider_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, thickness: Dp, color: Color) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		thickness.to_jni_value(),
		color.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


// Source /Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/pulltorefresh/PullToRefresh.kt
#[no_mangle]
pub extern "system" fn Java_com_elp_Indicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: PullToRefreshState, modifier: Modifier, color: Color) -> jobject {
	let args = vec![
		state.to_jni_value(),
		modifier.to_jni_value(),
		color.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}

