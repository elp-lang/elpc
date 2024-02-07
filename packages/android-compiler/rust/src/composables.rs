use jni::{objects::JClass,sys::jobject,JavaVM,JNIEnv};

use crate::types::{self, call_composable_function, Unit, Composable};

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


#[no_mangle]
pub extern "system" fn Java_com_elp_Text_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, text: String, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, minLines: Int, onTextLayout: fn(TextLayoutResult) -> Unit, style: TextStyle) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_Text_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, text: String, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, onTextLayout: fn(TextLayoutResult) -> Unit, style: TextStyle) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_Text_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, text: AnnotatedString, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, minLines: Int, inlineContent: Map<String, InlineTextContent>, onTextLayout: fn(TextLayoutResult) -> Unit, style: TextStyle) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_Text_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, text: AnnotatedString, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, inlineContent: Map<String, InlineTextContent>, onTextLayout: fn(TextLayoutResult) -> Unit, style: TextStyle) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_ProvideTextStyle_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: TextStyle, content: @Composable () -> Unit) -> jobject {
	let args = vec![
		value.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_DatePicker_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: DatePickerState, modifier: Modifier, dateFormatter: DatePickerFormatter, title: fn(Composable) -> Unit) -> jobject {
	let args = vec![
		state.to_jni_value(),
		modifier.to_jni_value(),
		dateFormatter.to_jni_value(),
		title.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, titleContentColor: Color, headlineContentColor: Color, weekdayContentColor: Color, subheadContentColor: Color, navigationContentColor: Color, yearContentColor: Color, disabledYearContentColor: Color, currentYearContentColor: Color, selectedYearContentColor: Color, disabledSelectedYearContentColor: Color, selectedYearContainerColor: Color, disabledSelectedYearContainerColor: Color, dayContentColor: Color, disabledDayContentColor: Color, selectedDayContentColor: Color, disabledSelectedDayContentColor: Color, selectedDayContainerColor: Color, disabledSelectedDayContainerColor: Color, todayContentColor: Color, todayDateBorderColor: Color, dayInSelectionRangeContentColor: Color, dayInSelectionRangeContainerColor: Color, dividerColor: Color, dateTextFieldColors: TextFieldColors) -> jobject {
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
		dateTextFieldColors.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_DatePickerTitle_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, displayMode: DisplayMode, modifier: Modifier) -> jobject {
	let args = vec![
		displayMode.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_Transition_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, inputState: InputPhase, focusedTextStyleColor: Color, unfocusedTextStyleColor: Color, contentColor: Composable, showLabel: Boolean, content: Composable, labelProgress: Float, labelTextStyleColor: Color, labelContentColor: Color, placeholderOpacity: Float, prefixSuffixOpacity: Float) -> jobject {
	let args = vec![
		inputState.to_jni_value(),
		focusedTextStyleColor.to_jni_value(),
		unfocusedTextStyleColor.to_jni_value(),
		contentColor.to_jni_value(),
		showLabel.to_jni_value(),
		content.to_jni_value(),
		labelProgress.to_jni_value(),
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


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, clockDialColor: Color, clockDialSelectedContentColor: Color, clockDialUnselectedContentColor: Color, selectorColor: Color, containerColor: Color, periodSelectorBorderColor: Color, periodSelectorSelectedContainerColor: Color, periodSelectorUnselectedContainerColor: Color, periodSelectorSelectedContentColor: Color, periodSelectorUnselectedContentColor: Color, timeSelectorSelectedContainerColor: Color, timeSelectorUnselectedContainerColor: Color, timeSelectorSelectedContentColor: Color, timeSelectorUnselectedContentColor: Color) -> jobject {
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
		timeSelectorUnselectedContentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_layoutType_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_itemColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_itemColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, textColor: Color, leadingIconColor: Color, trailingIconColor: Color, disabledTextColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color) -> jobject {
	let args = vec![
		textColor.to_jni_value(),
		leadingIconColor.to_jni_value(),
		trailingIconColor.to_jni_value(),
		disabledTextColor.to_jni_value(),
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


#[no_mangle]
pub extern "system" fn Java_com_elp_Slider_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: Float, onValueChange: (Float) -> jobject {
	let args = vec![
		value.to_jni_value(),
		onValueChange.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_RangeSlider_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: ClosedFloatingPointRange<Float>, onValueChange: (ClosedFloatingPointRange<Float>) -> jobject {
	let args = vec![
		value.to_jni_value(),
		onValueChange.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, thumbColor: Color, activeTrackColor: Color, activeTickColor: Color, inactiveTrackColor: Color, inactiveTickColor: Color, disabledThumbColor: Color, disabledActiveTrackColor: Color, disabledActiveTickColor: Color, disabledInactiveTrackColor: Color, disabledInactiveTickColor: Color) -> jobject {
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
		disabledInactiveTickColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


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


#[no_mangle]
pub extern "system" fn Java_com_elp_ElevatedCard_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, shape: Shape, colors: CardColors, elevation: CardElevation, content: Composable) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
		elevation.to_jni_value(),
		content.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ElevatedCard_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: CardColors, elevation: CardElevation, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
	let args = vec![
		onClick.to_jni_value(),
		modifier.to_jni_value(),
		enabled.to_jni_value(),
		shape.to_jni_value(),
		colors.to_jni_value(),
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


#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedCard_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, content: Composable) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedCard_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_cardElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let args = vec![
		defaultElevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedCardElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let args = vec![
		defaultElevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_outlinedCardElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let args = vec![
		defaultElevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_cardColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_cardColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedCardColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedCardColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_outlinedCardColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_outlinedCardColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


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


#[no_mangle]
pub extern "system" fn Java_com_elp_ElevatedButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_FilledTonalButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_TextButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_buttonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_buttonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_filledTonalButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_filledTonalButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_outlinedButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_outlinedButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledContentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_textButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_buttonElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp) -> jobject {
	let args = vec![
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


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedButtonElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp) -> jobject {
	let args = vec![
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


#[no_mangle]
pub extern "system" fn Java_com_elp_filledTonalButtonElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp) -> jobject {
	let args = vec![
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


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, disabledIconColor: Color, disabledTextColor: Color) -> jobject {
	let args = vec![
		selectedIconColor.to_jni_value(),
		selectedTextColor.to_jni_value(),
		indicatorColor.to_jni_value(),
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


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color) -> jobject {
	let args = vec![
		selectedIconColor.to_jni_value(),
		selectedTextColor.to_jni_value(),
		indicatorColor.to_jni_value(),
		unselectedIconColor.to_jni_value(),
		unselectedTextColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


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


#[no_mangle]
pub extern "system" fn Java_com_elp_Checkbox_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, checked: Boolean, onCheckedChange: fn(Boolean) -> Unit, modifier: Modifier, enabled: Boolean, colors: CheckboxColors, interactionSource: MutableInteractionSource) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, checkedColor: Color, uncheckedColor: Color, checkmarkColor: Color, disabledCheckedColor: Color, disabledUncheckedColor: Color, disabledIndeterminateColor: Color) -> jobject {
	let args = vec![
		checkedColor.to_jni_value(),
		uncheckedColor.to_jni_value(),
		checkmarkColor.to_jni_value(),
		disabledCheckedColor.to_jni_value(),
		disabledUncheckedColor.to_jni_value(),
		disabledIndeterminateColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_TextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: String, onValueChange: fn(String) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, prefix: Composable, suffix: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_TextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: TextFieldValue, onValueChange: fn(TextFieldValue) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, prefix: Composable, suffix: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_TextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: String, onValueChange: fn(String) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_TextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: TextFieldValue, onValueChange: fn(TextFieldValue) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, focusedTextColor: Color, unfocusedTextColor: Color, disabledTextColor: Color, errorTextColor: Color, focusedContainerColor: Color, unfocusedContainerColor: Color, disabledContainerColor: Color, errorContainerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedIndicatorColor: Color, unfocusedIndicatorColor: Color, disabledIndicatorColor: Color, errorIndicatorColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, focusedPlaceholderColor: Color, unfocusedPlaceholderColor: Color, disabledPlaceholderColor: Color, errorPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color) -> jobject {
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
		errorSuffixColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_FilledContainerBox_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedBorderContainerBox_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape, focusedBorderThickness: Dp, unfocusedBorderThickness: Dp) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, focusedTextColor: Color, unfocusedTextColor: Color, disabledTextColor: Color, errorTextColor: Color, focusedContainerColor: Color, unfocusedContainerColor: Color, disabledContainerColor: Color, errorContainerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedBorderColor: Color, unfocusedBorderColor: Color, disabledBorderColor: Color, errorBorderColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, focusedPlaceholderColor: Color, unfocusedPlaceholderColor: Color, disabledPlaceholderColor: Color, errorPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color) -> jobject {
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
		errorSuffixColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


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


#[no_mangle]
pub extern "system" fn Java_com_elp_elevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp) -> jobject {
	let args = vec![
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


#[no_mangle]
pub extern "system" fn Java_com_elp_loweredElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp) -> jobject {
	let args = vec![
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


#[no_mangle]
pub extern "system" fn Java_com_elp_IconToggleButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, checked: Boolean, onCheckedChange: fn(Boolean) -> Unit, modifier: Modifier, enabled: Boolean, colors: IconToggleButtonColors, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_FilledIconToggleButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, checked: Boolean, onCheckedChange: fn(Boolean) -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconToggleButtonColors, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_FilledTonalIconToggleButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, checked: Boolean, onCheckedChange: fn(Boolean) -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconToggleButtonColors, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedIconToggleButton_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, checked: Boolean, onCheckedChange: fn(Boolean) -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconToggleButtonColors, border: BorderStroke, interactionSource: MutableInteractionSource, content: Composable) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_filledIconButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_filledTonalIconButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_filledTonalIconToggleButtonColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_outlinedIconToggleButtonBorder_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, enabled: Boolean, checked: Boolean) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_outlinedIconButtonBorder_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, enabled: Boolean) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, disabledIconColor: Color, disabledTextColor: Color) -> jobject {
	let args = vec![
		selectedIconColor.to_jni_value(),
		selectedTextColor.to_jni_value(),
		indicatorColor.to_jni_value(),
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


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color) -> jobject {
	let args = vec![
		selectedIconColor.to_jni_value(),
		selectedTextColor.to_jni_value(),
		indicatorColor.to_jni_value(),
		unselectedIconColor.to_jni_value(),
		unselectedTextColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_rememberDrawerState_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, initialValue: DrawerValue, confirmStateChange: fn(DrawerValue) -> Boolean) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_NavigationDrawerItem_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, label: Composable, selected: Boolean, onClick: fn() -> Unit, modifier: Modifier, icon: fn(Composable) -> Unit, badge: fn(Composable) -> Unit, shape: Shape, colors: NavigationDrawerItemColors, interactionSource: MutableInteractionSource) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_iconColor_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean) -> jobject {
	let args = vec![
		selected.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_textColor_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean) -> jobject {
	let args = vec![
		selected.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_badgeColor_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean) -> jobject {
	let args = vec![
		selected.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_containerColor_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean) -> jobject {
	let args = vec![
		selected.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedContainerColor: Color, unselectedContainerColor: Color, selectedIconColor: Color, unselectedIconColor: Color, selectedTextColor: Color, unselectedTextColor: Color, selectedBadgeColor: Color, unselectedBadgeColor: Color) -> jobject {
	let args = vec![
		selectedContainerColor.to_jni_value(),
		unselectedContainerColor.to_jni_value(),
		selectedIconColor.to_jni_value(),
		unselectedIconColor.to_jni_value(),
		selectedTextColor.to_jni_value(),
		unselectedTextColor.to_jni_value(),
		selectedBadgeColor.to_jni_value(),
		unselectedBadgeColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_PrimaryTabRow_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, indicator: Composable) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_SecondaryTabRow_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, indicator: Composable) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_SecondaryScrollableTabRow_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedTabIndex: Int, modifier: Modifier, scrollState: ScrollState, containerColor: Color, contentColor: Color, edgePadding: Dp, indicator: Composable) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_ScrollableTabRow_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, edgePadding: Dp, indicator: Composable) -> jobject {
	let args = vec![
		selectedTabIndex.to_jni_value(),
		modifier.to_jni_value(),
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


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, activeContainerColor: Color, activeContentColor: Color, activeBorderColor: Color, inactiveContainerColor: Color, inactiveContentColor: Color, inactiveBorderColor: Color, disabledActiveContainerColor: Color, disabledActiveContentColor: Color, disabledActiveBorderColor: Color, disabledInactiveContainerColor: Color, disabledInactiveContentColor: Color, disabledInactiveBorderColor: Color) -> jobject {
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
		disabledInactiveBorderColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


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


#[no_mangle]
pub extern "system" fn Java_com_elp_Icon_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, active: Boolean, activeContent: Composable, inactiveContent: fn(Composable) -> Unit) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_Icon_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, imageVector: ImageVector, contentDescription: String?, modifier: Modifier, tint: Color) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_Icon_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, bitmap: ImageBitmap, contentDescription: String?, modifier: Modifier, tint: Color) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_Icon_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, painter: Painter, contentDescription: String?, modifier: Modifier, tint: Color) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_SmallTopAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, title: Composable, modifier: Modifier, navigationIcon: Composable, actions: Composable, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_CenterAlignedTopAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, title: Composable, modifier: Modifier, navigationIcon: Composable, actions: Composable, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_BottomAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, actions: Composable, modifier: Modifier, floatingActionButton: Composable, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets) -> jobject {
	let args = vec![
		actions.to_jni_value(),
		modifier.to_jni_value(),
		floatingActionButton.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		tonalElevation.to_jni_value(),
		contentPadding.to_jni_value(),
		windowInsets.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_BottomAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, actions: Composable, modifier: Modifier, floatingActionButton: Composable, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, scrollBehavior: BottomAppBarScrollBehavior) -> jobject {
	let args = vec![
		actions.to_jni_value(),
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


#[no_mangle]
pub extern "system" fn Java_com_elp_BottomAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, content: Composable) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		tonalElevation.to_jni_value(),
		contentPadding.to_jni_value(),
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


#[no_mangle]
pub extern "system" fn Java_com_elp_BottomAppBar_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, scrollBehavior: BottomAppBarScrollBehavior, content: Composable) -> jobject {
	let args = vec![
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


#[no_mangle]
pub extern "system" fn Java_com_elp_topAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_topAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		scrolledContainerColor.to_jni_value(),
		navigationIconContentColor.to_jni_value(),
		titleContentColor.to_jni_value(),
		actionIconContentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_centerAlignedTopAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_centerAlignedTopAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		scrolledContainerColor.to_jni_value(),
		navigationIconContentColor.to_jni_value(),
		titleContentColor.to_jni_value(),
		actionIconContentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_mediumTopAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_mediumTopAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		scrolledContainerColor.to_jni_value(),
		navigationIconContentColor.to_jni_value(),
		titleContentColor.to_jni_value(),
		actionIconContentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_largeTopAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_largeTopAppBarColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		scrolledContainerColor.to_jni_value(),
		navigationIconContentColor.to_jni_value(),
		titleContentColor.to_jni_value(),
		actionIconContentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_pinnedScrollBehavior_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: TopAppBarState, canScroll: fn() -> Boolean) -> jobject {
	let args = vec![
		state.to_jni_value(),
		canScroll.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_enterAlwaysScrollBehavior_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: TopAppBarState, canScroll: fn() -> Boolean, snapAnimationSpec: AnimationSpec<Float>, flingAnimationSpec: DecayAnimationSpec<Float>) -> jobject {
	let args = vec![
		state.to_jni_value(),
		canScroll.to_jni_value(),
		snapAnimationSpec.to_jni_value(),
		flingAnimationSpec.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_exitUntilCollapsedScrollBehavior_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: TopAppBarState, canScroll: fn() -> Boolean, snapAnimationSpec: AnimationSpec<Float>, flingAnimationSpec: DecayAnimationSpec<Float>) -> jobject {
	let args = vec![
		state.to_jni_value(),
		canScroll.to_jni_value(),
		snapAnimationSpec.to_jni_value(),
		flingAnimationSpec.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_rememberTopAppBarState_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, initialHeightOffsetLimit: Float, initialHeightOffset: Float, initialContentOffset: Float) -> jobject {
	let args = vec![
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


#[no_mangle]
pub extern "system" fn Java_com_elp_exitAlwaysScrollBehavior_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: BottomAppBarState, canScroll: fn() -> Boolean, snapAnimationSpec: AnimationSpec<Float>, flingAnimationSpec: DecayAnimationSpec<Float>) -> jobject {
	let args = vec![
		state.to_jni_value(),
		canScroll.to_jni_value(),
		snapAnimationSpec.to_jni_value(),
		flingAnimationSpec.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_rememberBottomAppBarState_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, initialHeightOffsetLimit: Float, initialHeightOffset: Float, initialContentOffset: Float) -> jobject {
	let args = vec![
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


#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedTextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: String, onValueChange: fn(String) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, prefix: Composable, suffix: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedTextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: TextFieldValue, onValueChange: fn(TextFieldValue) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, prefix: Composable, suffix: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedTextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: String, onValueChange: fn(String) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_OutlinedTextField_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, value: TextFieldValue, onValueChange: fn(TextFieldValue) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


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


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_colors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selectedColor: Color, unselectedColor: Color, disabledSelectedColor: Color, disabledUnselectedColor: Color) -> jobject {
	let args = vec![
		selectedColor.to_jni_value(),
		unselectedColor.to_jni_value(),
		disabledSelectedColor.to_jni_value(),
		disabledUnselectedColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


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


#[no_mangle]
pub extern "system" fn Java_com_elp_richTooltipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_richTooltipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, contentColor: Color, titleContentColor: Color, actionContentColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		contentColor.to_jni_value(),
		titleContentColor.to_jni_value(),
		actionContentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


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


#[no_mangle]
pub extern "system" fn Java_com_elp_AssistChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource) -> jobject {
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
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_AssistChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource) -> jobject {
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
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ElevatedAssistChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource) -> jobject {
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
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ElevatedAssistChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource) -> jobject {
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
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_FilterChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: SelectableChipColors, elevation: SelectableChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource) -> jobject {
	let args = vec![
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
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ElevatedFilterChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: SelectableChipColors, elevation: SelectableChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource) -> jobject {
	let args = vec![
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
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_InputChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, selected: Boolean, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, avatar: Composable, trailingIcon: Composable, shape: Shape, colors: SelectableChipColors, elevation: SelectableChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource) -> jobject {
	let args = vec![
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


#[no_mangle]
pub extern "system" fn Java_com_elp_SuggestionChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, icon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource) -> jobject {
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
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_SuggestionChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, icon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource) -> jobject {
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
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ElevatedSuggestionChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, icon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource) -> jobject {
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
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_ElevatedSuggestionChip_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, icon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource) -> jobject {
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
		interactionSource.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_assistChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_assistChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, labelColor: Color, leadingIconContentColor: Color, trailingIconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconContentColor: Color, disabledTrailingIconContentColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		labelColor.to_jni_value(),
		leadingIconContentColor.to_jni_value(),
		trailingIconContentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledLabelColor.to_jni_value(),
		disabledLeadingIconContentColor.to_jni_value(),
		disabledTrailingIconContentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_assistChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let args = vec![
		elevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedAssistChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedAssistChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, labelColor: Color, leadingIconContentColor: Color, trailingIconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconContentColor: Color, disabledTrailingIconContentColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		labelColor.to_jni_value(),
		leadingIconContentColor.to_jni_value(),
		trailingIconContentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledLabelColor.to_jni_value(),
		disabledLeadingIconContentColor.to_jni_value(),
		disabledTrailingIconContentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedAssistChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let args = vec![
		elevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_filterChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_filterChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, labelColor: Color, iconColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, selectedContainerColor: Color, disabledSelectedContainerColor: Color, selectedLabelColor: Color, selectedLeadingIconColor: Color, selectedTrailingIconColor: Color) -> jobject {
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
		selectedTrailingIconColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_filterChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let args = vec![
		elevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedFilterChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedFilterChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, labelColor: Color, iconColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, selectedContainerColor: Color, disabledSelectedContainerColor: Color, selectedLabelColor: Color, selectedLeadingIconColor: Color, selectedTrailingIconColor: Color) -> jobject {
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
		selectedTrailingIconColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedFilterChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let args = vec![
		elevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_inputChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_inputChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, labelColor: Color, leadingIconColor: Color, trailingIconColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, selectedContainerColor: Color, disabledSelectedContainerColor: Color, selectedLabelColor: Color, selectedLeadingIconColor: Color, selectedTrailingIconColor: Color) -> jobject {
	let args = vec![
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
		selectedTrailingIconColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_inputChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let args = vec![
		elevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_suggestionChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_suggestionChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, labelColor: Color, iconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledIconContentColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		labelColor.to_jni_value(),
		iconContentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledLabelColor.to_jni_value(),
		disabledIconContentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_suggestionChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let args = vec![
		elevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedSuggestionChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>) -> jobject {
	let args = vec![
		
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedSuggestionChipColors_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, containerColor: Color, labelColor: Color, iconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledIconContentColor: Color) -> jobject {
	let args = vec![
		containerColor.to_jni_value(),
		labelColor.to_jni_value(),
		iconContentColor.to_jni_value(),
		disabledContainerColor.to_jni_value(),
		disabledLabelColor.to_jni_value(),
		disabledIconContentColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_elevatedSuggestionChipElevation_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp) -> jobject {
	let args = vec![
		elevation.to_jni_value(),
		pressedElevation.to_jni_value(),
		focusedElevation.to_jni_value(),
		hoveredElevation.to_jni_value(),
		draggedElevation.to_jni_value(),
		disabledElevation.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_DateRangePicker_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, state: DateRangePickerState, modifier: Modifier, dateFormatter: DatePickerFormatter, title: fn(Composable) -> Unit) -> jobject {
	let args = vec![
		state.to_jni_value(),
		modifier.to_jni_value(),
		dateFormatter.to_jni_value(),
		title.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_DateRangePickerTitle_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, displayMode: DisplayMode, modifier: Modifier) -> jobject {
	let args = vec![
		displayMode.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


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


#[no_mangle]
pub extern "system" fn Java_com_elp_LinearProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, progress: fn() -> Float, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap, gapSize: Dp, drawStopIndicator: fn(DrawScope) -> Unit) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_LinearProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, progress: Float, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_LinearProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, progress: Float, modifier: Modifier, color: Color, trackColor: Color) -> jobject {
	let args = vec![
		progress.to_jni_value(),
		modifier.to_jni_value(),
		color.to_jni_value(),
		trackColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_LinearProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, color: Color, trackColor: Color) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		color.to_jni_value(),
		trackColor.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


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


#[no_mangle]
pub extern "system" fn Java_com_elp_CircularProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, progress: Float, modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap) -> jobject {
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


#[no_mangle]
pub extern "system" fn Java_com_elp_CircularProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, progress: Float, modifier: Modifier, color: Color, strokeWidth: Dp) -> jobject {
	let args = vec![
		progress.to_jni_value(),
		modifier.to_jni_value(),
		color.to_jni_value(),
		strokeWidth.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


#[no_mangle]
pub extern "system" fn Java_com_elp_CircularProgressIndicator_nativeMethod<'local>(env: JNIEnv<'local>, _class: JClass<'local>, modifier: Modifier, color: Color, strokeWidth: Dp) -> jobject {
	let args = vec![
		modifier.to_jni_value(),
		color.to_jni_value(),
		strokeWidth.to_jni_value()
	];

	return call_composable_function(
		env,
		_class,
		composable_name,
		args,
	);
}


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

