package com.elp

import androidx.compose.runtime.Composable

fun Label(label: @Composable CaretScope.() -> Unit, modifier: Modifier, interactionSource: MutableInteractionSource, isPersistent: Boolean, content: @Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for Label goes here
}

fun Text(text: String, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, minLines: Int, onTextLayout: ((TextLayoutResult) -> Unit), style: TextStyle)  : @Composable {
	// Your Jetpack Compose code for Text goes here
}

fun Text(text: String, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, onTextLayout: (TextLayoutResult) -> Unit, style: TextStyle)  : @Composable {
	// Your Jetpack Compose code for Text goes here
}

fun Text(text: AnnotatedString, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, minLines: Int, inlineContent: Map<String, InlineTextContent>, onTextLayout: (TextLayoutResult) -> Unit, style: TextStyle)  : @Composable {
	// Your Jetpack Compose code for Text goes here
}

fun Text(text: AnnotatedString, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, inlineContent: Map<String, InlineTextContent>, onTextLayout: (TextLayoutResult) -> Unit, style: TextStyle)  : @Composable {
	// Your Jetpack Compose code for Text goes here
}

fun ProvideTextStyle(value: TextStyle, content: @Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for ProvideTextStyle goes here
}

fun DatePicker(state: DatePickerState, modifier: Modifier, dateFormatter: DatePickerFormatter, title: (@Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for DatePicker goes here
}

fun colors()  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun colors(containerColor: Color, titleContentColor: Color, headlineContentColor: Color, weekdayContentColor: Color, subheadContentColor: Color, navigationContentColor: Color, yearContentColor: Color, disabledYearContentColor: Color, currentYearContentColor: Color, selectedYearContentColor: Color, disabledSelectedYearContentColor: Color, selectedYearContainerColor: Color, disabledSelectedYearContainerColor: Color, dayContentColor: Color, disabledDayContentColor: Color, selectedDayContentColor: Color, disabledSelectedDayContentColor: Color, selectedDayContainerColor: Color, disabledSelectedDayContainerColor: Color, todayContentColor: Color, todayDateBorderColor: Color, dayInSelectionRangeContentColor: Color, dayInSelectionRangeContainerColor: Color, dividerColor: Color, dateTextFieldColors: TextFieldColors)  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun DatePickerTitle(displayMode: DisplayMode, modifier: Modifier)  : @Composable {
	// Your Jetpack Compose code for DatePickerTitle goes here
}

fun Transition(inputState: InputPhase, focusedTextStyleColor: Color, unfocusedTextStyleColor: Color, contentColor: @Composable (InputPhase) -> Color, showLabel: Boolean, content: @Composable (, labelProgress: Float, labelTextStyleColor: Color, labelContentColor: Color, placeholderOpacity: Float, prefixSuffixOpacity: Float)  : @Composable {
	// Your Jetpack Compose code for Transition goes here
}

fun Scaffold(modifier: Modifier, topBar: @Composable () -> Unit, bottomBar: @Composable () -> Unit, snackbarHost: @Composable () -> Unit, floatingActionButton: @Composable () -> Unit, floatingActionButtonPosition: FabPosition, containerColor: Color, contentColor: Color, contentWindowInsets: WindowInsets, content: @Composable (PaddingValues) -> Unit)  : @Composable {
	// Your Jetpack Compose code for Scaffold goes here
}

fun colors()  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun colors(clockDialColor: Color, clockDialSelectedContentColor: Color, clockDialUnselectedContentColor: Color, selectorColor: Color, containerColor: Color, periodSelectorBorderColor: Color, periodSelectorSelectedContainerColor: Color, periodSelectorUnselectedContainerColor: Color, periodSelectorSelectedContentColor: Color, periodSelectorUnselectedContentColor: Color, timeSelectorSelectedContainerColor: Color, timeSelectorUnselectedContainerColor: Color, timeSelectorSelectedContentColor: Color, timeSelectorUnselectedContentColor: Color)  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun layoutType()  : @Composable {
	// Your Jetpack Compose code for layoutType goes here
}

fun itemColors()  : @Composable {
	// Your Jetpack Compose code for itemColors goes here
}

fun itemColors(textColor: Color, leadingIconColor: Color, trailingIconColor: Color, disabledTextColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color)  : @Composable {
	// Your Jetpack Compose code for itemColors goes here
}

fun Slider(value: Float, onValueChange: (Float)  : @Composable {
	// Your Jetpack Compose code for Slider goes here
}

fun RangeSlider(value: ClosedFloatingPointRange<Float>, onValueChange: (ClosedFloatingPointRange<Float>)  : @Composable {
	// Your Jetpack Compose code for RangeSlider goes here
}

fun colors()  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun colors(thumbColor: Color, activeTrackColor: Color, activeTickColor: Color, inactiveTrackColor: Color, inactiveTickColor: Color, disabledThumbColor: Color, disabledActiveTrackColor: Color, disabledActiveTickColor: Color, disabledInactiveTrackColor: Color, disabledInactiveTickColor: Color)  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun Thumb(interactionSource: MutableInteractionSource, modifier: Modifier, colors: SliderColors, enabled: Boolean, thumbSize: DpSize)  : @Composable {
	// Your Jetpack Compose code for Thumb goes here
}

fun Track(rangeSliderState: RangeSliderState, modifier: Modifier, colors: SliderColors, enabled: Boolean)  : @Composable {
	// Your Jetpack Compose code for Track goes here
}

fun Card(modifier: Modifier, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, content: @Composable ColumnScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for Card goes here
}

fun Card(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, interactionSource: MutableInteractionSource, content: @Composable ColumnScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for Card goes here
}

fun ElevatedCard(modifier: Modifier, shape: Shape, colors: CardColors, elevation: CardElevation, content: @Composable ColumnScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for ElevatedCard goes here
}

fun ElevatedCard(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: CardColors, elevation: CardElevation, interactionSource: MutableInteractionSource, content: @Composable ColumnScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for ElevatedCard goes here
}

fun OutlinedCard(modifier: Modifier, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, content: @Composable ColumnScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for OutlinedCard goes here
}

fun OutlinedCard(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, interactionSource: MutableInteractionSource, content: @Composable ColumnScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for OutlinedCard goes here
}

fun cardElevation(defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
	// Your Jetpack Compose code for cardElevation goes here
}

fun elevatedCardElevation(defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
	// Your Jetpack Compose code for elevatedCardElevation goes here
}

fun outlinedCardElevation(defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
	// Your Jetpack Compose code for outlinedCardElevation goes here
}

fun cardColors()  : @Composable {
	// Your Jetpack Compose code for cardColors goes here
}

fun cardColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color)  : @Composable {
	// Your Jetpack Compose code for cardColors goes here
}

fun elevatedCardColors()  : @Composable {
	// Your Jetpack Compose code for elevatedCardColors goes here
}

fun elevatedCardColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color)  : @Composable {
	// Your Jetpack Compose code for elevatedCardColors goes here
}

fun outlinedCardColors()  : @Composable {
	// Your Jetpack Compose code for outlinedCardColors goes here
}

fun outlinedCardColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color)  : @Composable {
	// Your Jetpack Compose code for outlinedCardColors goes here
}

fun outlinedCardBorder(enabled: Boolean)  : @Composable {
	// Your Jetpack Compose code for outlinedCardBorder goes here
}

fun Button(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: @Composable RowScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for Button goes here
}

fun ElevatedButton(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: @Composable RowScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for ElevatedButton goes here
}

fun FilledTonalButton(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: @Composable RowScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for FilledTonalButton goes here
}

fun OutlinedButton(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: @Composable RowScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for OutlinedButton goes here
}

fun TextButton(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: @Composable RowScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for TextButton goes here
}

fun buttonColors()  : @Composable {
	// Your Jetpack Compose code for buttonColors goes here
}

fun buttonColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color)  : @Composable {
	// Your Jetpack Compose code for buttonColors goes here
}

fun elevatedButtonColors()  : @Composable {
	// Your Jetpack Compose code for elevatedButtonColors goes here
}

fun elevatedButtonColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color)  : @Composable {
	// Your Jetpack Compose code for elevatedButtonColors goes here
}

fun filledTonalButtonColors()  : @Composable {
	// Your Jetpack Compose code for filledTonalButtonColors goes here
}

fun filledTonalButtonColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color)  : @Composable {
	// Your Jetpack Compose code for filledTonalButtonColors goes here
}

fun outlinedButtonColors()  : @Composable {
	// Your Jetpack Compose code for outlinedButtonColors goes here
}

fun outlinedButtonColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color)  : @Composable {
	// Your Jetpack Compose code for outlinedButtonColors goes here
}

fun textButtonColors()  : @Composable {
	// Your Jetpack Compose code for textButtonColors goes here
}

fun buttonElevation(defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp)  : @Composable {
	// Your Jetpack Compose code for buttonElevation goes here
}

fun elevatedButtonElevation(defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp)  : @Composable {
	// Your Jetpack Compose code for elevatedButtonElevation goes here
}

fun filledTonalButtonElevation(defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp)  : @Composable {
	// Your Jetpack Compose code for filledTonalButtonElevation goes here
}

fun NavigationBar(modifier: Modifier, containerColor: Color, contentColor: Color, tonalElevation: Dp, windowInsets: WindowInsets, content: @Composable RowScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for NavigationBar goes here
}

fun colors()  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun colors(selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, disabledIconColor: Color, disabledTextColor: Color)  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun colors(selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color)  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun MaterialTheme(colorScheme: ColorScheme, shapes: Shapes, typography: Typography, content: @Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for MaterialTheme goes here
}

fun Checkbox(checked: Boolean, onCheckedChange: ((Boolean) -> Unit)?, modifier: Modifier, enabled: Boolean, colors: CheckboxColors, interactionSource: MutableInteractionSource)  : @Composable {
	// Your Jetpack Compose code for Checkbox goes here
}

fun TriStateCheckbox(state: ToggleableState, onClick: (() -> Unit)?, modifier: Modifier, enabled: Boolean, colors: CheckboxColors, interactionSource: MutableInteractionSource)  : @Composable {
	// Your Jetpack Compose code for TriStateCheckbox goes here
}

fun colors()  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun colors(checkedColor: Color, uncheckedColor: Color, checkmarkColor: Color, disabledCheckedColor: Color, disabledUncheckedColor: Color, disabledIndeterminateColor: Color)  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun TextField(value: String, onValueChange: (String) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: @Composable (() -> Unit), placeholder: @Composable (() -> Unit), leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), prefix: @Composable (() -> Unit), suffix: @Composable (() -> Unit), supportingText: @Composable (() -> Unit), isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
	// Your Jetpack Compose code for TextField goes here
}

fun TextField(value: TextFieldValue, onValueChange: (TextFieldValue) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: @Composable (() -> Unit), placeholder: @Composable (() -> Unit), leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), prefix: @Composable (() -> Unit), suffix: @Composable (() -> Unit), supportingText: @Composable (() -> Unit), isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
	// Your Jetpack Compose code for TextField goes here
}

fun TextField(value: String, onValueChange: (String) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: @Composable (() -> Unit), placeholder: @Composable (() -> Unit), leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), supportingText: @Composable (() -> Unit), isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
	// Your Jetpack Compose code for TextField goes here
}

fun TextField(value: TextFieldValue, onValueChange: (TextFieldValue) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: @Composable (() -> Unit), placeholder: @Composable (() -> Unit), leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), supportingText: @Composable (() -> Unit), isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
	// Your Jetpack Compose code for TextField goes here
}

fun ContainerBox(enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape)  : @Composable {
	// Your Jetpack Compose code for ContainerBox goes here
}

fun colors()  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun colors(focusedTextColor: Color, unfocusedTextColor: Color, disabledTextColor: Color, errorTextColor: Color, focusedContainerColor: Color, unfocusedContainerColor: Color, disabledContainerColor: Color, errorContainerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedIndicatorColor: Color, unfocusedIndicatorColor: Color, disabledIndicatorColor: Color, errorIndicatorColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, focusedPlaceholderColor: Color, unfocusedPlaceholderColor: Color, disabledPlaceholderColor: Color, errorPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color)  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun FilledContainerBox(enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape)  : @Composable {
	// Your Jetpack Compose code for FilledContainerBox goes here
}

fun OutlinedBorderContainerBox(enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape, focusedBorderThickness: Dp, unfocusedBorderThickness: Dp)  : @Composable {
	// Your Jetpack Compose code for OutlinedBorderContainerBox goes here
}

fun ContainerBox(enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape, focusedBorderThickness: Dp, unfocusedBorderThickness: Dp)  : @Composable {
	// Your Jetpack Compose code for ContainerBox goes here
}

fun colors()  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun colors(focusedTextColor: Color, unfocusedTextColor: Color, disabledTextColor: Color, errorTextColor: Color, focusedContainerColor: Color, unfocusedContainerColor: Color, disabledContainerColor: Color, errorContainerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedBorderColor: Color, unfocusedBorderColor: Color, disabledBorderColor: Color, errorBorderColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, focusedPlaceholderColor: Color, unfocusedPlaceholderColor: Color, disabledPlaceholderColor: Color, errorPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color)  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun Tab(selected: Boolean, onClick: () -> Unit, modifier: Modifier, enabled: Boolean, text: @Composable (() -> Unit), icon: @Composable (() -> Unit), selectedContentColor: Color, unselectedContentColor: Color, interactionSource: MutableInteractionSource)  : @Composable {
	// Your Jetpack Compose code for Tab goes here
}

fun LeadingIconTab(selected: Boolean, onClick: () -> Unit, text: @Composable () -> Unit, icon: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, selectedContentColor: Color, unselectedContentColor: Color, interactionSource: MutableInteractionSource)  : @Composable {
	// Your Jetpack Compose code for LeadingIconTab goes here
}

fun Tab(selected: Boolean, onClick: () -> Unit, modifier: Modifier, enabled: Boolean, selectedContentColor: Color, unselectedContentColor: Color, interactionSource: MutableInteractionSource, content: @Composable ColumnScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for Tab goes here
}

fun FloatingActionButton(onClick: () -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for FloatingActionButton goes here
}

fun SmallFloatingActionButton(onClick: () -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for SmallFloatingActionButton goes here
}

fun LargeFloatingActionButton(onClick: () -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for LargeFloatingActionButton goes here
}

fun ExtendedFloatingActionButton(onClick: () -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content: @Composable RowScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for ExtendedFloatingActionButton goes here
}

fun ExtendedFloatingActionButton(text: @Composable () -> Unit, icon: @Composable () -> Unit, onClick: () -> Unit, modifier: Modifier, expanded: Boolean, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource)  : @Composable {
	// Your Jetpack Compose code for ExtendedFloatingActionButton goes here
}

fun elevation(defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp)  : @Composable {
	// Your Jetpack Compose code for elevation goes here
}

fun loweredElevation(defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp)  : @Composable {
	// Your Jetpack Compose code for loweredElevation goes here
}

fun IconButton(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, colors: IconButtonColors, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for IconButton goes here
}

fun IconToggleButton(checked: Boolean, onCheckedChange: (Boolean) -> Unit, modifier: Modifier, enabled: Boolean, colors: IconToggleButtonColors, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for IconToggleButton goes here
}

fun FilledIconButton(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconButtonColors, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for FilledIconButton goes here
}

fun FilledTonalIconButton(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconButtonColors, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for FilledTonalIconButton goes here
}

fun FilledIconToggleButton(checked: Boolean, onCheckedChange: (Boolean) -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconToggleButtonColors, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for FilledIconToggleButton goes here
}

fun FilledTonalIconToggleButton(checked: Boolean, onCheckedChange: (Boolean) -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconToggleButtonColors, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for FilledTonalIconToggleButton goes here
}

fun OutlinedIconButton(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconButtonColors, border: BorderStroke, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for OutlinedIconButton goes here
}

fun OutlinedIconToggleButton(checked: Boolean, onCheckedChange: (Boolean) -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconToggleButtonColors, border: BorderStroke, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for OutlinedIconToggleButton goes here
}

fun iconButtonColors()  : @Composable {
	// Your Jetpack Compose code for iconButtonColors goes here
}

fun filledIconButtonColors(containerColor: Color, contentColor: Color)  : @Composable {
	// Your Jetpack Compose code for filledIconButtonColors goes here
}

fun filledTonalIconButtonColors(containerColor: Color, contentColor: Color)  : @Composable {
	// Your Jetpack Compose code for filledTonalIconButtonColors goes here
}

fun filledTonalIconToggleButtonColors(containerColor: Color, contentColor: Color)  : @Composable {
	// Your Jetpack Compose code for filledTonalIconToggleButtonColors goes here
}

fun outlinedIconToggleButtonBorder(enabled: Boolean, checked: Boolean)  : @Composable {
	// Your Jetpack Compose code for outlinedIconToggleButtonBorder goes here
}

fun outlinedIconButtonBorder(enabled: Boolean)  : @Composable {
	// Your Jetpack Compose code for outlinedIconButtonBorder goes here
}

fun NavigationRail(modifier: Modifier, containerColor: Color, contentColor: Color, header: @Composable (ColumnScope.() -> Unit), windowInsets: WindowInsets, content: @Composable ColumnScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for NavigationRail goes here
}

fun NavigationRailItem(selected: Boolean, onClick: () -> Unit, icon: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, label: @Composable (() -> Unit), alwaysShowLabel: Boolean, colors: NavigationRailItemColors, interactionSource: MutableInteractionSource)  : @Composable {
	// Your Jetpack Compose code for NavigationRailItem goes here
}

fun colors()  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun colors(selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, disabledIconColor: Color, disabledTextColor: Color)  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun colors(selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color)  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun rememberDrawerState(initialValue: DrawerValue, confirmStateChange: (DrawerValue) -> Boolean)  : @Composable {
	// Your Jetpack Compose code for rememberDrawerState goes here
}

fun ModalNavigationDrawer(drawerContent: @Composable () -> Unit, modifier: Modifier, drawerState: DrawerState, gesturesEnabled: Boolean, scrimColor: Color, content: @Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for ModalNavigationDrawer goes here
}

fun DismissibleNavigationDrawer(drawerContent: @Composable () -> Unit, modifier: Modifier, drawerState: DrawerState, gesturesEnabled: Boolean, content: @Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for DismissibleNavigationDrawer goes here
}

fun PermanentNavigationDrawer(drawerContent: @Composable () -> Unit, modifier: Modifier, content: @Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for PermanentNavigationDrawer goes here
}

fun ModalDrawerSheet(modifier: Modifier, drawerShape: Shape, drawerContainerColor: Color, drawerContentColor: Color, drawerTonalElevation: Dp, windowInsets: WindowInsets, content: @Composable ColumnScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for ModalDrawerSheet goes here
}

fun DismissibleDrawerSheet(modifier: Modifier, drawerShape: Shape, drawerContainerColor: Color, drawerContentColor: Color, drawerTonalElevation: Dp, windowInsets: WindowInsets, content: @Composable ColumnScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for DismissibleDrawerSheet goes here
}

fun PermanentDrawerSheet(modifier: Modifier, drawerShape: Shape, drawerContainerColor: Color, drawerContentColor: Color, drawerTonalElevation: Dp, windowInsets: WindowInsets, content: @Composable ColumnScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for PermanentDrawerSheet goes here
}

fun NavigationDrawerItem(label: @Composable () -> Unit, selected: Boolean, onClick: () -> Unit, modifier: Modifier, icon: (@Composable () -> Unit), badge: (@Composable () -> Unit), shape: Shape, colors: NavigationDrawerItemColors, interactionSource: MutableInteractionSource)  : @Composable {
	// Your Jetpack Compose code for NavigationDrawerItem goes here
}

fun iconColor(selected: Boolean)  : @Composable {
	// Your Jetpack Compose code for iconColor goes here
}

fun textColor(selected: Boolean)  : @Composable {
	// Your Jetpack Compose code for textColor goes here
}

fun badgeColor(selected: Boolean)  : @Composable {
	// Your Jetpack Compose code for badgeColor goes here
}

fun containerColor(selected: Boolean)  : @Composable {
	// Your Jetpack Compose code for containerColor goes here
}

fun colors(selectedContainerColor: Color, unselectedContainerColor: Color, selectedIconColor: Color, unselectedIconColor: Color, selectedTextColor: Color, unselectedTextColor: Color, selectedBadgeColor: Color, unselectedBadgeColor: Color)  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun PrimaryTabRow(selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, indicator: @Composable TabIndicatorScope.()  : @Composable {
	// Your Jetpack Compose code for PrimaryTabRow goes here
}

fun SecondaryTabRow(selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, indicator: @Composable TabIndicatorScope.()  : @Composable {
	// Your Jetpack Compose code for SecondaryTabRow goes here
}

fun TabRow(selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, indicator: @Composable (tabPositions: List<TabPosition>)  : @Composable {
	// Your Jetpack Compose code for TabRow goes here
}

fun PrimaryScrollableTabRow(selectedTabIndex: Int, modifier: Modifier, scrollState: ScrollState, containerColor: Color, contentColor: Color, edgePadding: Dp, indicator: @Composable (tabPositions: List<TabPosition>)  : @Composable {
	// Your Jetpack Compose code for PrimaryScrollableTabRow goes here
}

fun SecondaryScrollableTabRow(selectedTabIndex: Int, modifier: Modifier, scrollState: ScrollState, containerColor: Color, contentColor: Color, edgePadding: Dp, indicator: @Composable (tabPositions: List<TabPosition>)  : @Composable {
	// Your Jetpack Compose code for SecondaryScrollableTabRow goes here
}

fun ScrollableTabRow(selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, edgePadding: Dp, indicator: @Composable (tabPositions: List<TabPosition>)  : @Composable {
	// Your Jetpack Compose code for ScrollableTabRow goes here
}

fun PrimaryIndicator(modifier: Modifier, width: Dp, height: Dp, color: Color, shape: Shape)  : @Composable {
	// Your Jetpack Compose code for PrimaryIndicator goes here
}

fun SecondaryIndicator(modifier: Modifier, height: Dp, color: Color)  : @Composable {
	// Your Jetpack Compose code for SecondaryIndicator goes here
}

fun colors()  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun colors(activeContainerColor: Color, activeContentColor: Color, activeBorderColor: Color, inactiveContainerColor: Color, inactiveContentColor: Color, inactiveBorderColor: Color, disabledActiveContainerColor: Color, disabledActiveContentColor: Color, disabledActiveBorderColor: Color, disabledInactiveContainerColor: Color, disabledInactiveContentColor: Color, disabledInactiveBorderColor: Color)  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun ActiveIcon()  : @Composable {
	// Your Jetpack Compose code for ActiveIcon goes here
}

fun Icon(active: Boolean, activeContent: @Composable () -> Unit, inactiveContent: (@Composable () -> Unit))  : @Composable {
	// Your Jetpack Compose code for Icon goes here
}

fun Icon(imageVector: ImageVector, contentDescription: String?, modifier: Modifier, tint: Color)  : @Composable {
	// Your Jetpack Compose code for Icon goes here
}

fun Icon(bitmap: ImageBitmap, contentDescription: String?, modifier: Modifier, tint: Color)  : @Composable {
	// Your Jetpack Compose code for Icon goes here
}

fun Icon(painter: Painter, contentDescription: String?, modifier: Modifier, tint: Color)  : @Composable {
	// Your Jetpack Compose code for Icon goes here
}

fun TopAppBar(title: @Composable () -> Unit, modifier: Modifier, navigationIcon: @Composable () -> Unit, actions: @Composable RowScope.() -> Unit, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior)  : @Composable {
	// Your Jetpack Compose code for TopAppBar goes here
}

fun SmallTopAppBar(title: @Composable () -> Unit, modifier: Modifier, navigationIcon: @Composable () -> Unit, actions: @Composable RowScope.() -> Unit, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior)  : @Composable {
	// Your Jetpack Compose code for SmallTopAppBar goes here
}

fun CenterAlignedTopAppBar(title: @Composable () -> Unit, modifier: Modifier, navigationIcon: @Composable () -> Unit, actions: @Composable RowScope.() -> Unit, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior)  : @Composable {
	// Your Jetpack Compose code for CenterAlignedTopAppBar goes here
}

fun MediumTopAppBar(title: @Composable () -> Unit, modifier: Modifier, navigationIcon: @Composable () -> Unit, actions: @Composable RowScope.() -> Unit, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior)  : @Composable {
	// Your Jetpack Compose code for MediumTopAppBar goes here
}

fun LargeTopAppBar(title: @Composable () -> Unit, modifier: Modifier, navigationIcon: @Composable () -> Unit, actions: @Composable RowScope.() -> Unit, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior)  : @Composable {
	// Your Jetpack Compose code for LargeTopAppBar goes here
}

fun BottomAppBar(actions: @Composable RowScope.() -> Unit, modifier: Modifier, floatingActionButton: @Composable (() -> Unit), containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets)  : @Composable {
	// Your Jetpack Compose code for BottomAppBar goes here
}

fun BottomAppBar(actions: @Composable RowScope.() -> Unit, modifier: Modifier, floatingActionButton: @Composable (() -> Unit), containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, scrollBehavior: BottomAppBarScrollBehavior)  : @Composable {
	// Your Jetpack Compose code for BottomAppBar goes here
}

fun BottomAppBar(modifier: Modifier, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, content: @Composable RowScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for BottomAppBar goes here
}

fun BottomAppBar(modifier: Modifier, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, scrollBehavior: BottomAppBarScrollBehavior, content: @Composable RowScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for BottomAppBar goes here
}

fun topAppBarColors()  : @Composable {
	// Your Jetpack Compose code for topAppBarColors goes here
}

fun topAppBarColors(containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color)  : @Composable {
	// Your Jetpack Compose code for topAppBarColors goes here
}

fun centerAlignedTopAppBarColors()  : @Composable {
	// Your Jetpack Compose code for centerAlignedTopAppBarColors goes here
}

fun centerAlignedTopAppBarColors(containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color)  : @Composable {
	// Your Jetpack Compose code for centerAlignedTopAppBarColors goes here
}

fun mediumTopAppBarColors()  : @Composable {
	// Your Jetpack Compose code for mediumTopAppBarColors goes here
}

fun mediumTopAppBarColors(containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color)  : @Composable {
	// Your Jetpack Compose code for mediumTopAppBarColors goes here
}

fun largeTopAppBarColors()  : @Composable {
	// Your Jetpack Compose code for largeTopAppBarColors goes here
}

fun largeTopAppBarColors(containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color)  : @Composable {
	// Your Jetpack Compose code for largeTopAppBarColors goes here
}

fun pinnedScrollBehavior(state: TopAppBarState, canScroll: () -> Boolean)  : @Composable {
	// Your Jetpack Compose code for pinnedScrollBehavior goes here
}

fun enterAlwaysScrollBehavior(state: TopAppBarState, canScroll: () -> Boolean, snapAnimationSpec: AnimationSpec<Float>, flingAnimationSpec: DecayAnimationSpec<Float>)  : @Composable {
	// Your Jetpack Compose code for enterAlwaysScrollBehavior goes here
}

fun exitUntilCollapsedScrollBehavior(state: TopAppBarState, canScroll: () -> Boolean, snapAnimationSpec: AnimationSpec<Float>, flingAnimationSpec: DecayAnimationSpec<Float>)  : @Composable {
	// Your Jetpack Compose code for exitUntilCollapsedScrollBehavior goes here
}

fun rememberTopAppBarState(initialHeightOffsetLimit: Float, initialHeightOffset: Float, initialContentOffset: Float)  : @Composable {
	// Your Jetpack Compose code for rememberTopAppBarState goes here
}

fun exitAlwaysScrollBehavior(state: BottomAppBarState, canScroll: () -> Boolean, snapAnimationSpec: AnimationSpec<Float>, flingAnimationSpec: DecayAnimationSpec<Float>)  : @Composable {
	// Your Jetpack Compose code for exitAlwaysScrollBehavior goes here
}

fun rememberBottomAppBarState(initialHeightOffsetLimit: Float, initialHeightOffset: Float, initialContentOffset: Float)  : @Composable {
	// Your Jetpack Compose code for rememberBottomAppBarState goes here
}

fun OutlinedTextField(value: String, onValueChange: (String) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: @Composable (() -> Unit), placeholder: @Composable (() -> Unit), leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), prefix: @Composable (() -> Unit), suffix: @Composable (() -> Unit), supportingText: @Composable (() -> Unit), isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
	// Your Jetpack Compose code for OutlinedTextField goes here
}

fun OutlinedTextField(value: TextFieldValue, onValueChange: (TextFieldValue) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: @Composable (() -> Unit), placeholder: @Composable (() -> Unit), leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), prefix: @Composable (() -> Unit), suffix: @Composable (() -> Unit), supportingText: @Composable (() -> Unit), isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
	// Your Jetpack Compose code for OutlinedTextField goes here
}

fun OutlinedTextField(value: String, onValueChange: (String) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: @Composable (() -> Unit), placeholder: @Composable (() -> Unit), leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), supportingText: @Composable (() -> Unit), isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
	// Your Jetpack Compose code for OutlinedTextField goes here
}

fun OutlinedTextField(value: TextFieldValue, onValueChange: (TextFieldValue) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: @Composable (() -> Unit), placeholder: @Composable (() -> Unit), leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), supportingText: @Composable (() -> Unit), isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
	// Your Jetpack Compose code for OutlinedTextField goes here
}

fun Snackbar(modifier: Modifier, action: @Composable (() -> Unit), dismissAction: @Composable (() -> Unit), actionOnNewLine: Boolean, shape: Shape, containerColor: Color, contentColor: Color, actionContentColor: Color, dismissActionContentColor: Color, content: @Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for Snackbar goes here
}

fun Snackbar(snackbarData: SnackbarData, modifier: Modifier, actionOnNewLine: Boolean, shape: Shape, containerColor: Color, contentColor: Color, actionColor: Color, actionContentColor: Color, dismissActionContentColor: Color)  : @Composable {
	// Your Jetpack Compose code for Snackbar goes here
}

fun ListItem(headlineContent: @Composable () -> Unit, modifier: Modifier, overlineContent: @Composable (() -> Unit), supportingContent: @Composable (() -> Unit), leadingContent: @Composable (() -> Unit), trailingContent: @Composable (() -> Unit), colors: ListItemColors, tonalElevation: Dp, shadowElevation: Dp)  : @Composable {
	// Your Jetpack Compose code for ListItem goes here
}

fun colors()  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun RadioButton(selected: Boolean, onClick: (() -> Unit)?, modifier: Modifier, enabled: Boolean, colors: RadioButtonColors, interactionSource: MutableInteractionSource)  : @Composable {
	// Your Jetpack Compose code for RadioButton goes here
}

fun colors()  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun colors(selectedColor: Color, unselectedColor: Color, disabledSelectedColor: Color, disabledUnselectedColor: Color)  : @Composable {
	// Your Jetpack Compose code for colors goes here
}

fun BadgedBox(badge: @Composable BoxScope.() -> Unit, modifier: Modifier, content: @Composable BoxScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for BadgedBox goes here
}

fun Badge(modifier: Modifier, containerColor: Color, contentColor: Color, content: @Composable (RowScope.() -> Unit))  : @Composable {
	// Your Jetpack Compose code for Badge goes here
}

fun richTooltipColors()  : @Composable {
	// Your Jetpack Compose code for richTooltipColors goes here
}

fun richTooltipColors(containerColor: Color, contentColor: Color, titleContentColor: Color, actionContentColor: Color)  : @Composable {
	// Your Jetpack Compose code for richTooltipColors goes here
}

fun rememberPlainTooltipPositionProvider(spacingBetweenTooltipAndAnchor: Dp)  : @Composable {
	// Your Jetpack Compose code for rememberPlainTooltipPositionProvider goes here
}

fun rememberRichTooltipPositionProvider(spacingBetweenTooltipAndAnchor: Dp)  : @Composable {
	// Your Jetpack Compose code for rememberRichTooltipPositionProvider goes here
}

fun AssistChip(onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource)  : @Composable {
	// Your Jetpack Compose code for AssistChip goes here
}

fun AssistChip(onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource)  : @Composable {
	// Your Jetpack Compose code for AssistChip goes here
}

fun ElevatedAssistChip(onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource)  : @Composable {
	// Your Jetpack Compose code for ElevatedAssistChip goes here
}

fun ElevatedAssistChip(onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource)  : @Composable {
	// Your Jetpack Compose code for ElevatedAssistChip goes here
}

fun FilterChip(selected: Boolean, onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), shape: Shape, colors: SelectableChipColors, elevation: SelectableChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource)  : @Composable {
	// Your Jetpack Compose code for FilterChip goes here
}

fun ElevatedFilterChip(selected: Boolean, onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), shape: Shape, colors: SelectableChipColors, elevation: SelectableChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource)  : @Composable {
	// Your Jetpack Compose code for ElevatedFilterChip goes here
}

fun InputChip(selected: Boolean, onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, leadingIcon: @Composable (() -> Unit), avatar: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), shape: Shape, colors: SelectableChipColors, elevation: SelectableChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource)  : @Composable {
	// Your Jetpack Compose code for InputChip goes here
}

fun SuggestionChip(onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, icon: @Composable (() -> Unit), shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource)  : @Composable {
	// Your Jetpack Compose code for SuggestionChip goes here
}

fun SuggestionChip(onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, icon: @Composable (() -> Unit), shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource)  : @Composable {
	// Your Jetpack Compose code for SuggestionChip goes here
}

fun ElevatedSuggestionChip(onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, icon: @Composable (() -> Unit), shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource)  : @Composable {
	// Your Jetpack Compose code for ElevatedSuggestionChip goes here
}

fun ElevatedSuggestionChip(onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, icon: @Composable (() -> Unit), shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource)  : @Composable {
	// Your Jetpack Compose code for ElevatedSuggestionChip goes here
}

fun assistChipColors()  : @Composable {
	// Your Jetpack Compose code for assistChipColors goes here
}

fun assistChipColors(containerColor: Color, labelColor: Color, leadingIconContentColor: Color, trailingIconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconContentColor: Color, disabledTrailingIconContentColor: Color)  : @Composable {
	// Your Jetpack Compose code for assistChipColors goes here
}

fun assistChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
	// Your Jetpack Compose code for assistChipElevation goes here
}

fun elevatedAssistChipColors()  : @Composable {
	// Your Jetpack Compose code for elevatedAssistChipColors goes here
}

fun elevatedAssistChipColors(containerColor: Color, labelColor: Color, leadingIconContentColor: Color, trailingIconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconContentColor: Color, disabledTrailingIconContentColor: Color)  : @Composable {
	// Your Jetpack Compose code for elevatedAssistChipColors goes here
}

fun elevatedAssistChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
	// Your Jetpack Compose code for elevatedAssistChipElevation goes here
}

fun filterChipColors()  : @Composable {
	// Your Jetpack Compose code for filterChipColors goes here
}

fun filterChipColors(containerColor: Color, labelColor: Color, iconColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, selectedContainerColor: Color, disabledSelectedContainerColor: Color, selectedLabelColor: Color, selectedLeadingIconColor: Color, selectedTrailingIconColor: Color)  : @Composable {
	// Your Jetpack Compose code for filterChipColors goes here
}

fun filterChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
	// Your Jetpack Compose code for filterChipElevation goes here
}

fun elevatedFilterChipColors()  : @Composable {
	// Your Jetpack Compose code for elevatedFilterChipColors goes here
}

fun elevatedFilterChipColors(containerColor: Color, labelColor: Color, iconColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, selectedContainerColor: Color, disabledSelectedContainerColor: Color, selectedLabelColor: Color, selectedLeadingIconColor: Color, selectedTrailingIconColor: Color)  : @Composable {
	// Your Jetpack Compose code for elevatedFilterChipColors goes here
}

fun elevatedFilterChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
	// Your Jetpack Compose code for elevatedFilterChipElevation goes here
}

fun inputChipColors()  : @Composable {
	// Your Jetpack Compose code for inputChipColors goes here
}

fun inputChipColors(containerColor: Color, labelColor: Color, leadingIconColor: Color, trailingIconColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, selectedContainerColor: Color, disabledSelectedContainerColor: Color, selectedLabelColor: Color, selectedLeadingIconColor: Color, selectedTrailingIconColor: Color)  : @Composable {
	// Your Jetpack Compose code for inputChipColors goes here
}

fun inputChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
	// Your Jetpack Compose code for inputChipElevation goes here
}

fun suggestionChipColors()  : @Composable {
	// Your Jetpack Compose code for suggestionChipColors goes here
}

fun suggestionChipColors(containerColor: Color, labelColor: Color, iconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledIconContentColor: Color)  : @Composable {
	// Your Jetpack Compose code for suggestionChipColors goes here
}

fun suggestionChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
	// Your Jetpack Compose code for suggestionChipElevation goes here
}

fun elevatedSuggestionChipColors()  : @Composable {
	// Your Jetpack Compose code for elevatedSuggestionChipColors goes here
}

fun elevatedSuggestionChipColors(containerColor: Color, labelColor: Color, iconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledIconContentColor: Color)  : @Composable {
	// Your Jetpack Compose code for elevatedSuggestionChipColors goes here
}

fun elevatedSuggestionChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
	// Your Jetpack Compose code for elevatedSuggestionChipElevation goes here
}

fun DateRangePicker(state: DateRangePickerState, modifier: Modifier, dateFormatter: DatePickerFormatter, title: (@Composable () -> Unit)  : @Composable {
	// Your Jetpack Compose code for DateRangePicker goes here
}

fun DateRangePickerTitle(displayMode: DisplayMode, modifier: Modifier)  : @Composable {
	// Your Jetpack Compose code for DateRangePickerTitle goes here
}

fun SnackbarHost(hostState: SnackbarHostState, modifier: Modifier, snackbar: @Composable (SnackbarData) -> Unit)  : @Composable {
	// Your Jetpack Compose code for SnackbarHost goes here
}

fun LinearProgressIndicator(progress: () -> Float, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap)  : @Composable {
	// Your Jetpack Compose code for LinearProgressIndicator goes here
}

fun LinearProgressIndicator(progress: () -> Float, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap, gapSize: Dp, drawStopIndicator: (DrawScope.() -> Unit)  : @Composable {
	// Your Jetpack Compose code for LinearProgressIndicator goes here
}

fun LinearProgressIndicator(modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap)  : @Composable {
	// Your Jetpack Compose code for LinearProgressIndicator goes here
}

fun LinearProgressIndicator(modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap, gapSize: Dp)  : @Composable {
	// Your Jetpack Compose code for LinearProgressIndicator goes here
}

fun LinearProgressIndicator(progress: Float, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap)  : @Composable {
	// Your Jetpack Compose code for LinearProgressIndicator goes here
}

fun LinearProgressIndicator(progress: Float, modifier: Modifier, color: Color, trackColor: Color)  : @Composable {
	// Your Jetpack Compose code for LinearProgressIndicator goes here
}

fun LinearProgressIndicator(modifier: Modifier, color: Color, trackColor: Color)  : @Composable {
	// Your Jetpack Compose code for LinearProgressIndicator goes here
}

fun CircularProgressIndicator(progress: () -> Float, modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap)  : @Composable {
	// Your Jetpack Compose code for CircularProgressIndicator goes here
}

fun CircularProgressIndicator(progress: () -> Float, modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap, gapSize: Dp)  : @Composable {
	// Your Jetpack Compose code for CircularProgressIndicator goes here
}

fun CircularProgressIndicator(modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap)  : @Composable {
	// Your Jetpack Compose code for CircularProgressIndicator goes here
}

fun CircularProgressIndicator(progress: Float, modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap)  : @Composable {
	// Your Jetpack Compose code for CircularProgressIndicator goes here
}

fun CircularProgressIndicator(progress: Float, modifier: Modifier, color: Color, strokeWidth: Dp)  : @Composable {
	// Your Jetpack Compose code for CircularProgressIndicator goes here
}

fun CircularProgressIndicator(modifier: Modifier, color: Color, strokeWidth: Dp)  : @Composable {
	// Your Jetpack Compose code for CircularProgressIndicator goes here
}

fun HorizontalDivider(modifier: Modifier, thickness: Dp, color: Color)  : @Composable {
	// Your Jetpack Compose code for HorizontalDivider goes here
}

fun VerticalDivider(modifier: Modifier, thickness: Dp, color: Color)  : @Composable {
	// Your Jetpack Compose code for VerticalDivider goes here
}

fun Divider(modifier: Modifier, thickness: Dp, color: Color)  : @Composable {
	// Your Jetpack Compose code for Divider goes here
}

fun Indicator(state: PullToRefreshState, modifier: Modifier, color: Color)  : @Composable {
	// Your Jetpack Compose code for Indicator goes here
}
