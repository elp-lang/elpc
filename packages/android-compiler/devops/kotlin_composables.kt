package com.elp

import androidx.compose.runtime.Composable

fun Label(label: Composable, modifier: Modifier, interactionSource: MutableInteractionSource, isPersistent: Boolean, content: Composable)  : @Composable {
}

fun Text(text: String, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, minLines: Int, onTextLayout: fn() -> Unit, style: TextStyle)  : @Composable {
}

fun Text(text: String, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, onTextLayout: fn() -> Unit, style: TextStyle)  : @Composable {
}

fun Text(text: AnnotatedString, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, minLines: Int, inlineContent: Map<String, InlineTextContent>, onTextLayout: fn() -> Unit, style: TextStyle)  : @Composable {
}

fun Text(text: AnnotatedString, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, inlineContent: Map<String, InlineTextContent>, onTextLayout: fn() -> Unit, style: TextStyle)  : @Composable {
}

fun ProvideTextStyle(value: TextStyle, content: Composable)  : @Composable {
}

fun DatePicker(state: DatePickerState, modifier: Modifier, dateFormatter: DatePickerFormatter, title: fn() -> Unit, headline: fn() -> Unit, showModeToggle: Boolean, colors: DatePickerColors)  : @Composable {
}

fun colors(containerColor: Color, titleContentColor: Color, headlineContentColor: Color, weekdayContentColor: Color, subheadContentColor: Color, navigationContentColor: Color, yearContentColor: Color, disabledYearContentColor: Color, currentYearContentColor: Color, selectedYearContentColor: Color, disabledSelectedYearContentColor: Color, selectedYearContainerColor: Color, disabledSelectedYearContainerColor: Color, dayContentColor: Color, disabledDayContentColor: Color, selectedDayContentColor: Color, disabledSelectedDayContentColor: Color, selectedDayContainerColor: Color, disabledSelectedDayContainerColor: Color, todayContentColor: Color, todayDateBorderColor: Color, dayInSelectionRangeContentColor: Color, dayInSelectionRangeContainerColor: Color, dividerColor: Color, dateTextFieldColors: TextFieldColors, defaultDatePickerColors: Composable)  : @Composable {
}

fun DatePickerTitle(displayMode: DisplayMode, modifier: Modifier)  : @Composable {
}

fun DatePickerHeadline(selectedDateMillis: Long, displayMode: DisplayMode, dateFormatter: DatePickerFormatter, modifier: Modifier)  : @Composable {
}

fun Transition(inputState: InputPhase, focusedTextStyleColor: Color, unfocusedTextStyleColor: Color, contentColor: Composable, showLabel: Boolean, content: Composable, labelTextStyleColor: Color, labelContentColor: Color, placeholderOpacity: Float, prefixSuffixOpacity: Float)  : @Composable {
}

fun Scaffold(modifier: Modifier, topBar: Composable, bottomBar: Composable, snackbarHost: Composable, floatingActionButton: Composable, floatingActionButtonPosition: FabPosition, containerColor: Color, contentColor: Color, contentWindowInsets: WindowInsets, content: Composable)  : @Composable {
}

fun colors(clockDialColor: Color, clockDialSelectedContentColor: Color, clockDialUnselectedContentColor: Color, selectorColor: Color, containerColor: Color, periodSelectorBorderColor: Color, periodSelectorSelectedContainerColor: Color, periodSelectorUnselectedContainerColor: Color, periodSelectorSelectedContentColor: Color, periodSelectorUnselectedContentColor: Color, timeSelectorSelectedContainerColor: Color, timeSelectorUnselectedContainerColor: Color, timeSelectorSelectedContentColor: Color, timeSelectorUnselectedContentColor: Color, defaultTimePickerColors: TimePickerColors
        get()  : @Composable {
}

fun layoutType(clockDialColor: Color, selectorColor: Color, containerColor: Color, periodSelectorBorderColor: Color, clockDialSelectedContentColor: Color, clockDialUnselectedContentColor: Color, periodSelectorSelectedContainerColor: Color, periodSelectorUnselectedContainerColor: Color, periodSelectorSelectedContentColor: Color, periodSelectorUnselectedContentColor: Color, timeSelectorSelectedContainerColor: Color, timeSelectorUnselectedContainerColor: Color, timeSelectorSelectedContentColor: Color, timeSelectorUnselectedContentColor: Color)  : @Composable {
}

fun itemColors(textColor: Color, leadingIconColor: Color, trailingIconColor: Color, disabledTextColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, defaultMenuItemColors: MenuItemColors
        get()  : @Composable {
}

fun Slider(value: Float, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, valueRange: ClosedFloatingPointRange<Float>, steps: Int, onValueChangeFinished: fn() -> Unit, colors: SliderColors, interactionSource: MutableInteractionSource)  : @Composable {
}

fun RangeSlider(value: ClosedFloatingPointRange<Float>, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, valueRange: ClosedFloatingPointRange<Float>, steps: Int, onValueChangeFinished: fn() -> Unit, colors: SliderColors)  : @Composable {
}

fun colors(thumbColor: Color, activeTrackColor: Color, activeTickColor: Color, inactiveTrackColor: Color, inactiveTickColor: Color, disabledThumbColor: Color, disabledActiveTrackColor: Color, disabledActiveTickColor: Color, disabledInactiveTrackColor: Color, disabledInactiveTickColor: Color, defaultSliderColors: SliderColors
        get()  : @Composable {
}

fun Thumb(interactionSource: MutableInteractionSource, modifier: Modifier, colors: SliderColors, enabled: Boolean, thumbSize: DpSize)  : @Composable {
}

fun Track(rangeSliderState: RangeSliderState, modifier: Modifier, colors: SliderColors, enabled: Boolean)  : @Composable {
}

fun Card(modifier: Modifier, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, content: Composable)  : @Composable {
}

fun Card(onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, interactionSource: MutableInteractionSource, content: Composable)  : @Composable {
}

fun ElevatedCard(modifier: Modifier, shape: Shape, colors: CardColors, elevation: CardElevation, content: Composable, https: //m3, https: //developer, also: * [Surface], onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: CardColors, elevation: CardElevation, interactionSource: MutableInteractionSource, content: Composable, https: //m3, also: * [Surface], shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, content: Composable, https: //m3, https: //developer, also: * [Surface], onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, interactionSource: MutableInteractionSource, content: Composable, shape: Composable, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, defaultCardColors: CardColors
        get()  : @Composable {
}

fun elevatedCardColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, defaultElevatedCardColors: CardColors
        get()  : @Composable {
}

fun outlinedCardColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, defaultOutlinedCardColors: CardColors
        get()  : @Composable {
}

fun outlinedCardBorder(enabled: Boolean)  : @Composable {
}

fun Button(onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: Composable)  : @Composable {
}

fun ElevatedButton(onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: Composable, https: //m3, https: //developer, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: Composable, https: //m3, https: //developer, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: Composable, https: //m3, https: //developer, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: Composable, shape: Composable, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, defaultButtonColors: ButtonColors
        get()  : @Composable {
}

fun elevatedButtonColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, defaultElevatedButtonColors: ButtonColors
        get()  : @Composable {
}

fun filledTonalButtonColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, defaultFilledTonalButtonColors: ButtonColors
        get()  : @Composable {
}

fun outlinedButtonColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, defaultOutlinedButtonColors: ButtonColors
        get()  : @Composable {
}

fun textButtonColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, defaultTextButtonColors: ButtonColors
        get()  : @Composable {
}

fun buttonElevation(defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp, outlinedButtonBorder: Composable, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp)  : @Composable {
}

fun NavigationBar(modifier: Modifier, containerColor: Color, contentColor: Color, tonalElevation: Dp, windowInsets: WindowInsets, content: Composable)  : @Composable {
}

fun colors(selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, disabledIconColor: Color, disabledTextColor: Color, defaultNavigationBarItemColors: NavigationBarItemColors
        get()  : @Composable {
}

fun colors(selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, selectedIconColor: Color, selectedTextColor: Color, selectedIndicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, disabledIconColor: Color, disabledTextColor: Color)  : @Composable {
}

fun MaterialTheme(colorScheme: ColorScheme, shapes: Shapes, typography: Typography, content: Composable)  : @Composable {
}

fun Checkbox(checked: Boolean, onCheckedChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, colors: CheckboxColors, interactionSource: MutableInteractionSource)  : @Composable {
}

fun TriStateCheckbox(state: ToggleableState, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, colors: CheckboxColors, interactionSource: MutableInteractionSource)  : @Composable {
}

fun colors(checkedColor: Color, uncheckedColor: Color, checkmarkColor: Color, disabledCheckedColor: Color, disabledUncheckedColor: Color, disabledIndeterminateColor: Color, defaultCheckboxColors: CheckboxColors
        get()  : @Composable {
}

fun TextField(value: String, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, prefix: Composable, suffix: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
}

fun TextField(value: TextFieldValue, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, prefix: Composable, suffix: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
}

fun TextField(value: String, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
}

fun TextField(value: TextFieldValue, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
}

fun ContainerBox(enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape)  : @Composable {
}

fun colors(focusedTextColor: Color, unfocusedTextColor: Color, disabledTextColor: Color, errorTextColor: Color, focusedContainerColor: Color, unfocusedContainerColor: Color, disabledContainerColor: Color, errorContainerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedIndicatorColor: Color, unfocusedIndicatorColor: Color, disabledIndicatorColor: Color, errorIndicatorColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, focusedPlaceholderColor: Color, unfocusedPlaceholderColor: Color, disabledPlaceholderColor: Color, errorPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color, defaultTextFieldColors: Composable)  : @Composable {
}

fun FilledContainerBox(enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape, enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape, focusedBorderThickness: Dp, unfocusedBorderThickness: Dp, start: Dp, end: Dp, top: Dp, bottom: Dp, start: Dp, top: Dp, end: Dp, bottom: Dp, start: Dp, top: Dp, end: Dp, bottom: Dp, focusedTextColor: Color, unfocusedTextColor: Color, disabledTextColor: Color, errorTextColor: Color, containerColor: Color, errorContainerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedIndicatorColor: Color, unfocusedIndicatorColor: Color, disabledIndicatorColor: Color, errorIndicatorColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, focusedPlaceholderColor: Color, unfocusedPlaceholderColor: Color, disabledPlaceholderColor: Color, errorPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color, focusedTextColor: Color, unfocusedTextColor: Color, disabledTextColor: Color, errorTextColor: Color, containerColor: Color, errorContainerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedBorderColor: Color, unfocusedBorderColor: Color, disabledBorderColor: Color, errorBorderColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, focusedPlaceholderColor: Color, unfocusedPlaceholderColor: Color, disabledPlaceholderColor: Color, errorPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color, value: String, innerTextField: Composable, enabled: Boolean, singleLine: Boolean, visualTransformation: VisualTransformation, interactionSource: InteractionSource, isError: Boolean, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, prefix: Composable, suffix: Composable, supportingText: Composable, shape: Shape, colors: TextFieldColors, contentPadding: PaddingValues)  : @Composable {
}

fun textFieldColors(textColor: Color, disabledTextColor: Color, containerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedIndicatorColor: Color, unfocusedIndicatorColor: Color, disabledIndicatorColor: Color, errorIndicatorColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, placeholderColor: Color, disabledPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color, textColor: Color, disabledTextColor: Color, containerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedBorderColor: Color, unfocusedBorderColor: Color, disabledBorderColor: Color, errorBorderColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, placeholderColor: Color, disabledPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color, value: String, innerTextField: Composable, enabled: Boolean, singleLine: Boolean, visualTransformation: VisualTransformation, interactionSource: InteractionSource, isError: Boolean, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, supportingText: Composable, shape: Shape, colors: TextFieldColors, contentPadding: PaddingValues)  : @Composable {
}

fun ContainerBox(enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape, focusedBorderThickness: Dp, unfocusedBorderThickness: Dp)  : @Composable {
}

fun colors(focusedTextColor: Color, unfocusedTextColor: Color, disabledTextColor: Color, errorTextColor: Color, focusedContainerColor: Color, unfocusedContainerColor: Color, disabledContainerColor: Color, errorContainerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedBorderColor: Color, unfocusedBorderColor: Color, disabledBorderColor: Color, errorBorderColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, focusedPlaceholderColor: Color, unfocusedPlaceholderColor: Color, disabledPlaceholderColor: Color, errorPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color, defaultOutlinedTextFieldColors: Composable)  : @Composable {
}

fun Tab(selected: Boolean, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, text: Composable, icon: Composable, selectedContentColor: Color, unselectedContentColor: Color, interactionSource: MutableInteractionSource)  : @Composable {
}

fun LeadingIconTab(selected: Boolean, onClick: fn() -> Unit, text: Composable, icon: Composable, modifier: Modifier, enabled: Boolean, selectedContentColor: Color, unselectedContentColor: Color, interactionSource: MutableInteractionSource)  : @Composable {
}

fun Tab(selected: Boolean, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, selectedContentColor: Color, unselectedContentColor: Color, interactionSource: MutableInteractionSource, content: Composable)  : @Composable {
}

fun FloatingActionButton(onClick: fn() -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content: Composable)  : @Composable {
}

fun SmallFloatingActionButton(onClick: fn() -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content: Composable)  : @Composable {
}

fun LargeFloatingActionButton(onClick: fn() -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content: Composable)  : @Composable {
}

fun ExtendedFloatingActionButton(onClick: fn() -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content: Composable)  : @Composable {
}

fun ExtendedFloatingActionButton(text: Composable, icon: Composable, onClick: fn() -> Unit, modifier: Modifier, expanded: Boolean, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource)  : @Composable {
}

fun elevation(defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp)  : @Composable {
}

fun IconButton(onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, colors: IconButtonColors, interactionSource: MutableInteractionSource, content: Composable)  : @Composable {
}

fun IconToggleButton(checked: Boolean, onCheckedChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, colors: IconToggleButtonColors, interactionSource: MutableInteractionSource, content: Composable)  : @Composable {
}

fun FilledIconButton(onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconButtonColors, interactionSource: MutableInteractionSource, content: Composable)  : @Composable {
}

fun FilledTonalIconButton(onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconButtonColors, interactionSource: MutableInteractionSource, content: Composable)  : @Composable {
}

fun FilledIconToggleButton(checked: Boolean, onCheckedChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconToggleButtonColors, interactionSource: MutableInteractionSource, content: Composable)  : @Composable {
}

fun FilledTonalIconToggleButton(checked: Boolean, onCheckedChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconToggleButtonColors, interactionSource: MutableInteractionSource, content: Composable)  : @Composable {
}

fun OutlinedIconButton(onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconButtonColors, border: BorderStroke, interactionSource: MutableInteractionSource, content: Composable)  : @Composable {
}

fun OutlinedIconToggleButton(checked: Boolean, onCheckedChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconToggleButtonColors, border: BorderStroke, interactionSource: MutableInteractionSource, content: Composable)  : @Composable {
}

fun iconButtonColors()  : @Composable {
}

fun iconButtonColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, defaultIconButtonColors: Composable)  : @Composable {
}

fun iconToggleButtonColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, checkedContainerColor: Color, checkedContentColor: Color, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, checkedContainerColor: Color, checkedContentColor: Color, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, checkedContainerColor: Color, checkedContentColor: Color, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color, checkedContainerColor: Color, checkedContentColor: Color, enabled: Boolean, checked: Boolean): BorderStroke? {
        if (checked)  : @Composable {
}

fun outlinedIconButtonBorder(enabled: Boolean): BorderStroke {
        val color: Color)  : @Composable {
}

fun NavigationRail(modifier: Modifier, containerColor: Color, contentColor: Color, header: Composable, windowInsets: WindowInsets, content: Composable)  : @Composable {
}

fun NavigationRailItem(selected: Boolean, onClick: fn() -> Unit, icon: Composable, modifier: Modifier, enabled: Boolean, label: Composable, alwaysShowLabel: Boolean, colors: NavigationRailItemColors, interactionSource: MutableInteractionSource)  : @Composable {
}

fun colors(selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, disabledIconColor: Color, disabledTextColor: Color, defaultNavigationRailItemColors: NavigationRailItemColors
        get()  : @Composable {
}

fun colors(selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, selectedIconColor: Color, selectedTextColor: Color, selectedIndicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, disabledIconColor: Color, disabledTextColor: Color)  : @Composable {
}

fun rememberDrawerState(initialValue: DrawerValue, confirmStateChange: fn() -> Boolean)  : @Composable {
}

fun ModalNavigationDrawer(drawerContent: Composable, modifier: Modifier, drawerState: DrawerState, gesturesEnabled: Boolean, scrimColor: Color, content: Composable)  : @Composable {
}

fun DismissibleNavigationDrawer(drawerContent: Composable, modifier: Modifier, drawerState: DrawerState, gesturesEnabled: Boolean, content: Composable)  : @Composable {
}

fun PermanentNavigationDrawer(drawerContent: Composable, modifier: Modifier, content: Composable)  : @Composable {
}

fun ModalDrawerSheet(modifier: Modifier, drawerShape: Shape, drawerContainerColor: Color, drawerContentColor: Color, drawerTonalElevation: Dp, windowInsets: WindowInsets, content: Composable)  : @Composable {
}

fun DismissibleDrawerSheet(modifier: Modifier, drawerShape: Shape, drawerContainerColor: Color, drawerContentColor: Color, drawerTonalElevation: Dp, windowInsets: WindowInsets, content: Composable)  : @Composable {
}

fun PermanentDrawerSheet(modifier: Modifier, drawerShape: Shape, drawerContainerColor: Color, drawerContentColor: Color, drawerTonalElevation: Dp, windowInsets: WindowInsets, content: Composable)  : @Composable {
}

fun NavigationDrawerItem(label: Composable, selected: Boolean, onClick: fn() -> Unit, modifier: Modifier, icon: fn() -> Unit, badge: fn() -> Unit, shape: Shape, colors: NavigationDrawerItemColors, interactionSource: MutableInteractionSource)  : @Composable {
}

fun iconColor(selected: Boolean): State<Color>

    /**
     * Represents the text color for this item, selected: Boolean): State<Color>

    /**
     * Represents the badge color for this item, selected: Boolean): State<Color>

    /**
     * Represents the container color for this item, selected: Boolean): State<Color>
}

/** Defaults used in [NavigationDrawerItem], unselectedContainerColor: Color, selectedIconColor: Color, unselectedIconColor: Color, selectedTextColor: Color, unselectedTextColor: Color, selectedBadgeColor: Color, unselectedBadgeColor: Color, selectedIconColor: Color, unselectedIconColor: Color, selectedTextColor: Color, unselectedTextColor: Color, selectedContainerColor: Color, unselectedContainerColor: Color, selectedBadgeColor: Color, unselectedBadgeColor: Composable, b: Float, pos: Float), open: Boolean, onClose: fn() -> Unit, fraction: fn() -> Float, color: Color)  : @Composable {
}

fun PrimaryTabRow(selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, indicator: Composable, divider: Composable, tabs: Composable)  : @Composable {
}

fun SecondaryTabRow(selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, indicator: Composable, divider: Composable, tabs: Composable)  : @Composable {
}

fun TabRow(selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, indicator: Composable)  : @Composable {
}

fun PrimaryScrollableTabRow(selectedTabIndex: Int, modifier: Modifier, scrollState: ScrollState, containerColor: Color, contentColor: Color, edgePadding: Dp, indicator: Composable)  : @Composable {
}

fun SecondaryScrollableTabRow(selectedTabIndex: Int, modifier: Modifier, scrollState: ScrollState, containerColor: Color, contentColor: Color, edgePadding: Dp, indicator: Composable, divider: Composable, tabs: Composable)  : @Composable {
}

fun ScrollableTabRow(selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, edgePadding: Dp, indicator: Composable, divider: Composable, tabs: Composable)  : @Composable {
}

fun PrimaryIndicator(modifier: Modifier, width: Dp, height: Dp, color: Color, shape: Shape)  : @Composable {
}

fun SecondaryIndicator(modifier: Modifier, height: Dp, color: Color)  : @Composable {
}

fun colors(activeContainerColor: Color, activeContentColor: Color, activeBorderColor: Color, inactiveContainerColor: Color, inactiveContentColor: Color, inactiveBorderColor: Color, disabledActiveContainerColor: Color, disabledActiveContentColor: Color, disabledActiveBorderColor: Color, disabledInactiveContainerColor: Color, disabledInactiveContentColor: Color, disabledInactiveBorderColor: Color, defaultSegmentedButtonColors: SegmentedButtonColors
        get()  : @Composable {
}

fun ActiveIcon()  : @Composable {
}

fun Icon(active: Boolean, activeContent: Composable, inactiveContent: fn() -> Unit)  : @Composable {
}

fun Icon(imageVector: ImageVector, contentDescription: String, modifier: Modifier, tint: Color)  : @Composable {
}

fun Icon(bitmap: ImageBitmap, contentDescription: String, modifier: Modifier, tint: Color)  : @Composable {
}

fun Icon(painter: Painter, contentDescription: String, modifier: Modifier, tint: Color)  : @Composable {
}

fun TopAppBar(title: Composable, modifier: Modifier, navigationIcon: Composable, actions: Composable, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior)  : @Composable {
}

fun SmallTopAppBar(title: Composable, modifier: Modifier, navigationIcon: Composable, actions: Composable, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior, https: //m3, like: * @sample androidx, title: Composable, modifier: Modifier, navigationIcon: Composable, actions: Composable, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior)  : @Composable {
}

fun MediumTopAppBar(title: Composable, modifier: Modifier, navigationIcon: Composable, actions: Composable, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior)  : @Composable {
}

fun LargeTopAppBar(title: Composable, modifier: Modifier, navigationIcon: Composable, actions: Composable, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior)  : @Composable {
}

fun BottomAppBar(actions: Composable, modifier: Modifier, floatingActionButton: Composable, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, https: //m3, also: [Surface], modifier: Modifier, floatingActionButton: Composable, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, scrollBehavior: BottomAppBarScrollBehavior)  : @Composable {
}

fun BottomAppBar(modifier: Modifier, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, content: Composable, https: //m3, also: [Surface], modifier: Modifier, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, scrollBehavior: BottomAppBarScrollBehavior, content: Composable)  : @Composable {
}

fun topAppBarColors(containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color, defaultTopAppBarColors: TopAppBarColors
        get()  : @Composable {
}

fun smallTopAppBarColors(containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color, windowInsets: Composable, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color, defaultCenterAlignedTopAppBarColors: TopAppBarColors
        get()  : @Composable {
}

fun mediumTopAppBarColors(containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color, defaultMediumTopAppBarColors: TopAppBarColors
        get()  : @Composable {
}

fun largeTopAppBarColors(containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color, defaultLargeTopAppBarColors: TopAppBarColors
        get()  : @Composable {
}

fun pinnedScrollBehavior(state: TopAppBarState, canScroll: fn() -> Boolean, state: TopAppBarState, canScroll: fn() -> Boolean, snapAnimationSpec: AnimationSpec<Float>, flingAnimationSpec: DecayAnimationSpec<Float>, state: TopAppBarState, canScroll: fn() -> Boolean, snapAnimationSpec: AnimationSpec<Float>, flingAnimationSpec: DecayAnimationSpec<Float>, initialHeightOffsetLimit: Float, initialHeightOffset: Float, initialContentOffset: Float)  : @Composable {
}

fun exitAlwaysScrollBehavior(state: BottomAppBarState, canScroll: fn() -> Boolean, snapAnimationSpec: AnimationSpec<Float>, flingAnimationSpec: DecayAnimationSpec<Float>, initialHeightOffsetLimit: Float, initialHeightOffset: Float, initialContentOffset: Float)  : @Composable {
}

fun OutlinedTextField(value: String, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, prefix: Composable, suffix: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
}

fun OutlinedTextField(value: TextFieldValue, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, prefix: Composable, suffix: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
}

fun OutlinedTextField(value: String, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
}

fun OutlinedTextField(value: TextFieldValue, onValueChange: fn() -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: Composable, placeholder: Composable, leadingIcon: Composable, trailingIcon: Composable, supportingText: Composable, isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
}

fun Snackbar(modifier: Modifier, action: Composable, dismissAction: Composable, actionOnNewLine: Boolean, shape: Shape, containerColor: Color, contentColor: Color, actionContentColor: Color, dismissActionContentColor: Color, content: Composable)  : @Composable {
}

fun Snackbar(snackbarData: SnackbarData, modifier: Modifier, actionOnNewLine: Boolean, shape: Shape, containerColor: Color, contentColor: Color, actionColor: Color, actionContentColor: Color, dismissActionContentColor: Color)  : @Composable {
}

fun ListItem(headlineContent: Composable, modifier: Modifier, overlineContent: Composable, supportingContent: Composable, leadingContent: Composable, trailingContent: Composable, colors: ListItemColors, tonalElevation: Dp, shadowElevation: Dp)  : @Composable {
}

fun colors(containerColor: Color, headlineColor: Color, leadingIconColor: Color, overlineColor: Color, supportingColor: Color, trailingIconColor: Color, disabledHeadlineColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, containerColor: Color, headlineColor: Color, leadingIconColor: Color, overlineColor: Color, supportingTextColor: Color, trailingIconColor: Color, disabledHeadlineColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color)  : @Composable {
}

fun DragHandle(modifier: Modifier, width: Dp, height: Dp, shape: Shape, color: Color)  : @Composable {
}

fun colors(checkedThumbColor: Color, checkedTrackColor: Color, checkedBorderColor: Color, checkedIconColor: Color, uncheckedThumbColor: Color, uncheckedTrackColor: Color, uncheckedBorderColor: Color, uncheckedIconColor: Color, disabledCheckedThumbColor: Color, disabledCheckedTrackColor: Color, disabledCheckedBorderColor: Color, disabledCheckedIconColor: Color, disabledUncheckedThumbColor: Color, disabledUncheckedTrackColor: Color, disabledUncheckedBorderColor: Color, disabledUncheckedIconColor: Color, defaultSwitchColors: SwitchColors
        get()  : @Composable {
}

fun RadioButton(selected: Boolean, onClick: fn() -> Unit, modifier: Modifier, enabled: Boolean, colors: RadioButtonColors, interactionSource: MutableInteractionSource)  : @Composable {
}

fun colors(selectedColor: Color, unselectedColor: Color, disabledSelectedColor: Color, disabledUnselectedColor: Color, defaultRadioButtonColors: RadioButtonColors
        get()  : @Composable {
}

fun BadgedBox(badge: Composable, modifier: Modifier, content: Composable)  : @Composable {
}

fun Badge(modifier: Modifier, containerColor: Color, contentColor: Color, content: Composable)  : @Composable {
}

fun richTooltipColors(containerColor: Color, contentColor: Color, titleContentColor: Color, actionContentColor: Color, defaultRichTooltipColors: RichTooltipColors
        get()  : @Composable {
}

fun rememberPlainTooltipPositionProvider(spacingBetweenTooltipAndAnchor: Dp)  : @Composable {
}

fun rememberRichTooltipPositionProvider(spacingBetweenTooltipAndAnchor: Dp)  : @Composable {
}

fun AssistChip(onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource, https: //m3, https: //developer, AssistChip: * @sample androidx, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource, https: //m3, https: //developer, icon: * @sample androidx, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource, https: //m3, https: //developer, icon: * @sample androidx, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource, https: //m3, https: //developer, icon: * @sample androidx, selected: Boolean, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: SelectableChipColors, elevation: SelectableChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource, https: //m3, https: //developer, icon: * @sample androidx, selected: Boolean, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: SelectableChipColors, elevation: SelectableChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource, https: //m3, https: //developer, icon: * @sample androidx, selected: Boolean, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, leadingIcon: Composable, avatar: Composable, trailingIcon: Composable, shape: Shape, colors: SelectableChipColors, elevation: SelectableChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource)  : @Composable {
}

fun SuggestionChip(onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, icon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource, https: //m3, https: //developer, icon: * @sample androidx, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, icon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource, https: //m3, https: //developer, icon: * @sample androidx, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, icon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource, https: //m3, https: //developer, icon: * @sample androidx, onClick: fn() -> Unit, label: Composable, modifier: Modifier, enabled: Boolean, icon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource, containerColor: Color, labelColor: Color, leadingIconContentColor: Color, trailingIconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconContentColor: Color, disabledTrailingIconContentColor: Color, defaultAssistChipColors: ChipColors
        get()  : @Composable {
}

fun assistChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, enabled: Boolean, borderColor: Color, disabledBorderColor: Color, borderWidth: Dp, borderColor: Color, disabledBorderColor: Color, borderWidth: Dp, containerColor: Color, labelColor: Color, leadingIconContentColor: Color, trailingIconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconContentColor: Color, disabledTrailingIconContentColor: Color, defaultElevatedAssistChipColors: ChipColors
        get()  : @Composable {
}

fun elevatedAssistChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, shape: Composable, labelColor: Color, iconColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, selectedContainerColor: Color, disabledSelectedContainerColor: Color, selectedLabelColor: Color, selectedLeadingIconColor: Color, selectedTrailingIconColor: Color, defaultFilterChipColors: SelectableChipColors
        get()  : @Composable {
}

fun filterChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, enabled: Boolean, selected: Boolean, borderColor: Color, selectedBorderColor: Color, disabledBorderColor: Color, disabledSelectedBorderColor: Color, borderWidth: Dp, selectedBorderWidth: Dp)  : @Composable {
}

fun elevatedFilterChipColors(containerColor: Color, labelColor: Color, iconColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, selectedContainerColor: Color, disabledSelectedContainerColor: Color, selectedLabelColor: Color, selectedLeadingIconColor: Color, selectedTrailingIconColor: Color, defaultElevatedFilterChipColors: SelectableChipColors
        get()  : @Composable {
}

fun elevatedFilterChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, shape: Composable, containerColor: Color, labelColor: Color, leadingIconColor: Color, trailingIconColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, selectedContainerColor: Color, disabledSelectedContainerColor: Color, selectedLabelColor: Color, selectedLeadingIconColor: Color, selectedTrailingIconColor: Color, defaultInputChipColors: SelectableChipColors
        get()  : @Composable {
}

fun inputChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, enabled: Boolean, selected: Boolean, borderColor: Color, selectedBorderColor: Color, disabledBorderColor: Color, disabledSelectedBorderColor: Color, borderWidth: Dp, selectedBorderWidth: Dp)  : @Composable {
}

fun suggestionChipColors(containerColor: Color, labelColor: Color, iconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledIconContentColor: Color, elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, enabled: Boolean, borderColor: Color, disabledBorderColor: Color, borderWidth: Dp, borderColor: Color, disabledBorderColor: Color, borderWidth: Dp, containerColor: Color, labelColor: Color, iconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledIconContentColor: Color, defaultElevatedSuggestionChipColors: ChipColors
        get()  : @Composable {
}

fun elevatedSuggestionChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp, shape: Composable, onClick: fn() -> Unit, enabled: Boolean, label: Composable, labelTextStyle: TextStyle, labelColor: Color, leadingIcon: Composable, trailingIcon: Composable, shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, minHeight: Dp, paddingValues: PaddingValues, interactionSource: MutableInteractionSource)  : @Composable {
}

fun DateRangePicker(state: DateRangePickerState, modifier: Modifier, dateFormatter: DatePickerFormatter, title: fn() -> Unit, headline: fn() -> Unit, showModeToggle: Boolean, colors: DatePickerColors)  : @Composable {
}

fun DateRangePickerTitle(displayMode: DisplayMode, modifier: Modifier)  : @Composable {
}

fun DateRangePickerHeadline(selectedStartDateMillis: Long, selectedEndDateMillis: Long, displayMode: DisplayMode, dateFormatter: DatePickerFormatter, modifier: Modifier)  : @Composable {
}

fun SnackbarHost(hostState: SnackbarHostState, modifier: Modifier, snackbar: Composable)  : @Composable {
}

fun LinearProgressIndicator(progress: fn() -> Float, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap)  : @Composable {
}

fun LinearProgressIndicator(progress: fn() -> Float, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap, gapSize: Dp, drawStopIndicator: (DrawScope)  : @Composable {
}

fun LinearProgressIndicator(modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap)  : @Composable {
}

fun LinearProgressIndicator(modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap, gapSize: Dp)  : @Composable {
}

fun LinearProgressIndicator(progress: Float, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap, progress: Float, modifier: Modifier, color: Color, trackColor: Color, modifier: Modifier, color: Color, trackColor: Color, startFraction: Float, endFraction: Float, color: Color, strokeWidth: Float, strokeCap: StrokeCap)  : @Composable {
}

fun CircularProgressIndicator(progress: fn() -> Float, modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap)  : @Composable {
}

fun CircularProgressIndicator(progress: fn() -> Float, modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap, gapSize: Dp)  : @Composable {
}

fun CircularProgressIndicator(modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap)  : @Composable {
}

fun CircularProgressIndicator(progress: Float, modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap, progress: Float, modifier: Modifier, color: Color, strokeWidth: Dp, modifier: Modifier, color: Color, strokeWidth: Dp, startAngle: Float, sweep: Float, color: Color, stroke: Stroke)  : @Composable {
}

fun HorizontalDivider(modifier: Modifier, thickness: Dp, color: Color)  : @Composable {
}

fun VerticalDivider(modifier: Modifier, thickness: Dp, color: Color)  : @Composable {
}

fun Divider(modifier: Modifier, thickness: Dp, color: Color)  : @Composable {
}

fun Indicator(state: PullToRefreshState, modifier: Modifier, color: Color)  : @Composable {
}
