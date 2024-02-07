package com.elp

import androidx.compose.runtime.Composable

fun Label(label: @Composable CaretScope.() -> Unit, modifier: Modifier, interactionSource: MutableInteractionSource, isPersistent: Boolean, content: @Composable () -> Unit)  : @Composable {
}

fun Text(text: String, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, minLines: Int, onTextLayout: ((TextLayoutResult) -> Unit), style: TextStyle)  : @Composable {
}

fun Text(text: String, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, onTextLayout: (TextLayoutResult) -> Unit, style: TextStyle)  : @Composable {
}

fun Text(text: AnnotatedString, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, minLines: Int, inlineContent: Map<String, InlineTextContent>, onTextLayout: (TextLayoutResult) -> Unit, style: TextStyle)  : @Composable {
}

fun Text(text: AnnotatedString, modifier: Modifier, color: Color, fontSize: TextUnit, fontStyle: FontStyle, fontWeight: FontWeight, fontFamily: FontFamily, letterSpacing: TextUnit, textDecoration: TextDecoration, textAlign: TextAlign, lineHeight: TextUnit, overflow: TextOverflow, softWrap: Boolean, maxLines: Int, inlineContent: Map<String, InlineTextContent>, onTextLayout: (TextLayoutResult) -> Unit, style: TextStyle)  : @Composable {
}

fun ProvideTextStyle(value: TextStyle, content: @Composable () -> Unit)  : @Composable {
}

fun DatePicker(state: DatePickerState, modifier: Modifier, dateFormatter: DatePickerFormatter, title: (@Composable () -> Unit)  : @Composable {
}

fun colors()  : @Composable {
}

fun colors(containerColor: Color, titleContentColor: Color, headlineContentColor: Color, weekdayContentColor: Color, subheadContentColor: Color, navigationContentColor: Color, yearContentColor: Color, disabledYearContentColor: Color, currentYearContentColor: Color, selectedYearContentColor: Color, disabledSelectedYearContentColor: Color, selectedYearContainerColor: Color, disabledSelectedYearContainerColor: Color, dayContentColor: Color, disabledDayContentColor: Color, selectedDayContentColor: Color, disabledSelectedDayContentColor: Color, selectedDayContainerColor: Color, disabledSelectedDayContainerColor: Color, todayContentColor: Color, todayDateBorderColor: Color, dayInSelectionRangeContentColor: Color, dayInSelectionRangeContainerColor: Color, dividerColor: Color, dateTextFieldColors: TextFieldColors)  : @Composable {
}

fun DatePickerTitle(displayMode: DisplayMode, modifier: Modifier)  : @Composable {
}

fun Transition(inputState: InputPhase, focusedTextStyleColor: Color, unfocusedTextStyleColor: Color, contentColor: @Composable (InputPhase) -> Color, showLabel: Boolean, content: @Composable (, labelProgress: Float, labelTextStyleColor: Color, labelContentColor: Color, placeholderOpacity: Float, prefixSuffixOpacity: Float)  : @Composable {
}

fun Scaffold(modifier: Modifier, topBar: @Composable () -> Unit, bottomBar: @Composable () -> Unit, snackbarHost: @Composable () -> Unit, floatingActionButton: @Composable () -> Unit, floatingActionButtonPosition: FabPosition, containerColor: Color, contentColor: Color, contentWindowInsets: WindowInsets, content: @Composable (PaddingValues) -> Unit)  : @Composable {
}

fun colors()  : @Composable {
}

fun colors(clockDialColor: Color, clockDialSelectedContentColor: Color, clockDialUnselectedContentColor: Color, selectorColor: Color, containerColor: Color, periodSelectorBorderColor: Color, periodSelectorSelectedContainerColor: Color, periodSelectorUnselectedContainerColor: Color, periodSelectorSelectedContentColor: Color, periodSelectorUnselectedContentColor: Color, timeSelectorSelectedContainerColor: Color, timeSelectorUnselectedContainerColor: Color, timeSelectorSelectedContentColor: Color, timeSelectorUnselectedContentColor: Color)  : @Composable {
}

fun layoutType()  : @Composable {
}

fun itemColors()  : @Composable {
}

fun itemColors(textColor: Color, leadingIconColor: Color, trailingIconColor: Color, disabledTextColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color)  : @Composable {
}

fun Slider(value: Float, onValueChange: (Float)  : @Composable {
}

fun RangeSlider(value: ClosedFloatingPointRange<Float>, onValueChange: (ClosedFloatingPointRange<Float>)  : @Composable {
}

fun colors()  : @Composable {
}

fun colors(thumbColor: Color, activeTrackColor: Color, activeTickColor: Color, inactiveTrackColor: Color, inactiveTickColor: Color, disabledThumbColor: Color, disabledActiveTrackColor: Color, disabledActiveTickColor: Color, disabledInactiveTrackColor: Color, disabledInactiveTickColor: Color)  : @Composable {
}

fun Thumb(interactionSource: MutableInteractionSource, modifier: Modifier, colors: SliderColors, enabled: Boolean, thumbSize: DpSize)  : @Composable {
}

fun Track(rangeSliderState: RangeSliderState, modifier: Modifier, colors: SliderColors, enabled: Boolean)  : @Composable {
}

fun Card(modifier: Modifier, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, content: @Composable ColumnScope.() -> Unit)  : @Composable {
}

fun Card(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, interactionSource: MutableInteractionSource, content: @Composable ColumnScope.() -> Unit)  : @Composable {
}

fun ElevatedCard(modifier: Modifier, shape: Shape, colors: CardColors, elevation: CardElevation, content: @Composable ColumnScope.() -> Unit)  : @Composable {
}

fun ElevatedCard(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: CardColors, elevation: CardElevation, interactionSource: MutableInteractionSource, content: @Composable ColumnScope.() -> Unit)  : @Composable {
}

fun OutlinedCard(modifier: Modifier, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, content: @Composable ColumnScope.() -> Unit)  : @Composable {
}

fun OutlinedCard(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: CardColors, elevation: CardElevation, border: BorderStroke, interactionSource: MutableInteractionSource, content: @Composable ColumnScope.() -> Unit)  : @Composable {
}

fun cardElevation(defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
}

fun elevatedCardElevation(defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
}

fun outlinedCardElevation(defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
}

fun cardColors()  : @Composable {
}

fun cardColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color)  : @Composable {
}

fun elevatedCardColors()  : @Composable {
}

fun elevatedCardColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color)  : @Composable {
}

fun outlinedCardColors()  : @Composable {
}

fun outlinedCardColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color)  : @Composable {
}

fun outlinedCardBorder(enabled: Boolean)  : @Composable {
}

fun Button(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: @Composable RowScope.() -> Unit)  : @Composable {
}

fun ElevatedButton(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: @Composable RowScope.() -> Unit)  : @Composable {
}

fun FilledTonalButton(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: @Composable RowScope.() -> Unit)  : @Composable {
}

fun OutlinedButton(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: @Composable RowScope.() -> Unit)  : @Composable {
}

fun TextButton(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: ButtonColors, elevation: ButtonElevation, border: BorderStroke, contentPadding: PaddingValues, interactionSource: MutableInteractionSource, content: @Composable RowScope.() -> Unit)  : @Composable {
}

fun buttonColors()  : @Composable {
}

fun buttonColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color)  : @Composable {
}

fun elevatedButtonColors()  : @Composable {
}

fun elevatedButtonColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color)  : @Composable {
}

fun filledTonalButtonColors()  : @Composable {
}

fun filledTonalButtonColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color)  : @Composable {
}

fun outlinedButtonColors()  : @Composable {
}

fun outlinedButtonColors(containerColor: Color, contentColor: Color, disabledContainerColor: Color, disabledContentColor: Color)  : @Composable {
}

fun textButtonColors()  : @Composable {
}

fun buttonElevation(defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp)  : @Composable {
}

fun elevatedButtonElevation(defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp)  : @Composable {
}

fun filledTonalButtonElevation(defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, disabledElevation: Dp)  : @Composable {
}

fun NavigationBar(modifier: Modifier, containerColor: Color, contentColor: Color, tonalElevation: Dp, windowInsets: WindowInsets, content: @Composable RowScope.() -> Unit)  : @Composable {
}

fun colors()  : @Composable {
}

fun colors(selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, disabledIconColor: Color, disabledTextColor: Color)  : @Composable {
}

fun colors(selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color)  : @Composable {
}

fun MaterialTheme(colorScheme: ColorScheme, shapes: Shapes, typography: Typography, content: @Composable () -> Unit)  : @Composable {
}

fun Checkbox(checked: Boolean, onCheckedChange: ((Boolean) -> Unit)?, modifier: Modifier, enabled: Boolean, colors: CheckboxColors, interactionSource: MutableInteractionSource)  : @Composable {
}

fun TriStateCheckbox(state: ToggleableState, onClick: (() -> Unit)?, modifier: Modifier, enabled: Boolean, colors: CheckboxColors, interactionSource: MutableInteractionSource)  : @Composable {
}

fun colors()  : @Composable {
}

fun colors(checkedColor: Color, uncheckedColor: Color, checkmarkColor: Color, disabledCheckedColor: Color, disabledUncheckedColor: Color, disabledIndeterminateColor: Color)  : @Composable {
}

fun TextField(value: String, onValueChange: (String) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: @Composable (() -> Unit), placeholder: @Composable (() -> Unit), leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), prefix: @Composable (() -> Unit), suffix: @Composable (() -> Unit), supportingText: @Composable (() -> Unit), isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
}

fun TextField(value: TextFieldValue, onValueChange: (TextFieldValue) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: @Composable (() -> Unit), placeholder: @Composable (() -> Unit), leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), prefix: @Composable (() -> Unit), suffix: @Composable (() -> Unit), supportingText: @Composable (() -> Unit), isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
}

fun TextField(value: String, onValueChange: (String) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: @Composable (() -> Unit), placeholder: @Composable (() -> Unit), leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), supportingText: @Composable (() -> Unit), isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
}

fun TextField(value: TextFieldValue, onValueChange: (TextFieldValue) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: @Composable (() -> Unit), placeholder: @Composable (() -> Unit), leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), supportingText: @Composable (() -> Unit), isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
}

fun ContainerBox(enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape)  : @Composable {
}

fun colors()  : @Composable {
}

fun colors(focusedTextColor: Color, unfocusedTextColor: Color, disabledTextColor: Color, errorTextColor: Color, focusedContainerColor: Color, unfocusedContainerColor: Color, disabledContainerColor: Color, errorContainerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedIndicatorColor: Color, unfocusedIndicatorColor: Color, disabledIndicatorColor: Color, errorIndicatorColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, focusedPlaceholderColor: Color, unfocusedPlaceholderColor: Color, disabledPlaceholderColor: Color, errorPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color)  : @Composable {
}

fun FilledContainerBox(enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape)  : @Composable {
}

fun OutlinedBorderContainerBox(enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape, focusedBorderThickness: Dp, unfocusedBorderThickness: Dp)  : @Composable {
}

fun ContainerBox(enabled: Boolean, isError: Boolean, interactionSource: InteractionSource, colors: TextFieldColors, shape: Shape, focusedBorderThickness: Dp, unfocusedBorderThickness: Dp)  : @Composable {
}

fun colors()  : @Composable {
}

fun colors(focusedTextColor: Color, unfocusedTextColor: Color, disabledTextColor: Color, errorTextColor: Color, focusedContainerColor: Color, unfocusedContainerColor: Color, disabledContainerColor: Color, errorContainerColor: Color, cursorColor: Color, errorCursorColor: Color, selectionColors: TextSelectionColors, focusedBorderColor: Color, unfocusedBorderColor: Color, disabledBorderColor: Color, errorBorderColor: Color, focusedLeadingIconColor: Color, unfocusedLeadingIconColor: Color, disabledLeadingIconColor: Color, errorLeadingIconColor: Color, focusedTrailingIconColor: Color, unfocusedTrailingIconColor: Color, disabledTrailingIconColor: Color, errorTrailingIconColor: Color, focusedLabelColor: Color, unfocusedLabelColor: Color, disabledLabelColor: Color, errorLabelColor: Color, focusedPlaceholderColor: Color, unfocusedPlaceholderColor: Color, disabledPlaceholderColor: Color, errorPlaceholderColor: Color, focusedSupportingTextColor: Color, unfocusedSupportingTextColor: Color, disabledSupportingTextColor: Color, errorSupportingTextColor: Color, focusedPrefixColor: Color, unfocusedPrefixColor: Color, disabledPrefixColor: Color, errorPrefixColor: Color, focusedSuffixColor: Color, unfocusedSuffixColor: Color, disabledSuffixColor: Color, errorSuffixColor: Color)  : @Composable {
}

fun Tab(selected: Boolean, onClick: () -> Unit, modifier: Modifier, enabled: Boolean, text: @Composable (() -> Unit), icon: @Composable (() -> Unit), selectedContentColor: Color, unselectedContentColor: Color, interactionSource: MutableInteractionSource)  : @Composable {
}

fun LeadingIconTab(selected: Boolean, onClick: () -> Unit, text: @Composable () -> Unit, icon: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, selectedContentColor: Color, unselectedContentColor: Color, interactionSource: MutableInteractionSource)  : @Composable {
}

fun Tab(selected: Boolean, onClick: () -> Unit, modifier: Modifier, enabled: Boolean, selectedContentColor: Color, unselectedContentColor: Color, interactionSource: MutableInteractionSource, content: @Composable ColumnScope.() -> Unit)  : @Composable {
}

fun FloatingActionButton(onClick: () -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
}

fun SmallFloatingActionButton(onClick: () -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
}

fun LargeFloatingActionButton(onClick: () -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
}

fun ExtendedFloatingActionButton(onClick: () -> Unit, modifier: Modifier, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource, content: @Composable RowScope.() -> Unit)  : @Composable {
}

fun ExtendedFloatingActionButton(text: @Composable () -> Unit, icon: @Composable () -> Unit, onClick: () -> Unit, modifier: Modifier, expanded: Boolean, shape: Shape, containerColor: Color, contentColor: Color, elevation: FloatingActionButtonElevation, interactionSource: MutableInteractionSource)  : @Composable {
}

fun elevation(defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp)  : @Composable {
}

fun loweredElevation(defaultElevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp)  : @Composable {
}

fun IconButton(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, colors: IconButtonColors, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
}

fun IconToggleButton(checked: Boolean, onCheckedChange: (Boolean) -> Unit, modifier: Modifier, enabled: Boolean, colors: IconToggleButtonColors, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
}

fun FilledIconButton(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconButtonColors, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
}

fun FilledTonalIconButton(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconButtonColors, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
}

fun FilledIconToggleButton(checked: Boolean, onCheckedChange: (Boolean) -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconToggleButtonColors, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
}

fun FilledTonalIconToggleButton(checked: Boolean, onCheckedChange: (Boolean) -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconToggleButtonColors, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
}

fun OutlinedIconButton(onClick: () -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconButtonColors, border: BorderStroke, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
}

fun OutlinedIconToggleButton(checked: Boolean, onCheckedChange: (Boolean) -> Unit, modifier: Modifier, enabled: Boolean, shape: Shape, colors: IconToggleButtonColors, border: BorderStroke, interactionSource: MutableInteractionSource, content: @Composable () -> Unit)  : @Composable {
}

fun iconButtonColors()  : @Composable {
}

fun filledIconButtonColors(containerColor: Color, contentColor: Color)  : @Composable {
}

fun filledTonalIconButtonColors(containerColor: Color, contentColor: Color)  : @Composable {
}

fun filledTonalIconToggleButtonColors(containerColor: Color, contentColor: Color)  : @Composable {
}

fun outlinedIconToggleButtonBorder(enabled: Boolean, checked: Boolean)  : @Composable {
}

fun outlinedIconButtonBorder(enabled: Boolean)  : @Composable {
}

fun NavigationRail(modifier: Modifier, containerColor: Color, contentColor: Color, header: @Composable (ColumnScope.() -> Unit), windowInsets: WindowInsets, content: @Composable ColumnScope.() -> Unit)  : @Composable {
}

fun NavigationRailItem(selected: Boolean, onClick: () -> Unit, icon: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, label: @Composable (() -> Unit), alwaysShowLabel: Boolean, colors: NavigationRailItemColors, interactionSource: MutableInteractionSource)  : @Composable {
}

fun colors()  : @Composable {
}

fun colors(selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color, disabledIconColor: Color, disabledTextColor: Color)  : @Composable {
}

fun colors(selectedIconColor: Color, selectedTextColor: Color, indicatorColor: Color, unselectedIconColor: Color, unselectedTextColor: Color)  : @Composable {
}

fun rememberDrawerState(initialValue: DrawerValue, confirmStateChange: (DrawerValue) -> Boolean)  : @Composable {
}

fun ModalNavigationDrawer(drawerContent: @Composable () -> Unit, modifier: Modifier, drawerState: DrawerState, gesturesEnabled: Boolean, scrimColor: Color, content: @Composable () -> Unit)  : @Composable {
}

fun DismissibleNavigationDrawer(drawerContent: @Composable () -> Unit, modifier: Modifier, drawerState: DrawerState, gesturesEnabled: Boolean, content: @Composable () -> Unit)  : @Composable {
}

fun PermanentNavigationDrawer(drawerContent: @Composable () -> Unit, modifier: Modifier, content: @Composable () -> Unit)  : @Composable {
}

fun ModalDrawerSheet(modifier: Modifier, drawerShape: Shape, drawerContainerColor: Color, drawerContentColor: Color, drawerTonalElevation: Dp, windowInsets: WindowInsets, content: @Composable ColumnScope.() -> Unit)  : @Composable {
}

fun DismissibleDrawerSheet(modifier: Modifier, drawerShape: Shape, drawerContainerColor: Color, drawerContentColor: Color, drawerTonalElevation: Dp, windowInsets: WindowInsets, content: @Composable ColumnScope.() -> Unit)  : @Composable {
}

fun PermanentDrawerSheet(modifier: Modifier, drawerShape: Shape, drawerContainerColor: Color, drawerContentColor: Color, drawerTonalElevation: Dp, windowInsets: WindowInsets, content: @Composable ColumnScope.() -> Unit)  : @Composable {
}

fun NavigationDrawerItem(label: @Composable () -> Unit, selected: Boolean, onClick: () -> Unit, modifier: Modifier, icon: (@Composable () -> Unit), badge: (@Composable () -> Unit), shape: Shape, colors: NavigationDrawerItemColors, interactionSource: MutableInteractionSource)  : @Composable {
}

fun iconColor(selected: Boolean)  : @Composable {
}

fun textColor(selected: Boolean)  : @Composable {
}

fun badgeColor(selected: Boolean)  : @Composable {
}

fun containerColor(selected: Boolean)  : @Composable {
}

fun colors(selectedContainerColor: Color, unselectedContainerColor: Color, selectedIconColor: Color, unselectedIconColor: Color, selectedTextColor: Color, unselectedTextColor: Color, selectedBadgeColor: Color, unselectedBadgeColor: Color)  : @Composable {
}

fun PrimaryTabRow(selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, indicator: @Composable TabIndicatorScope.()  : @Composable {
}

fun SecondaryTabRow(selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, indicator: @Composable TabIndicatorScope.()  : @Composable {
}

fun TabRow(selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, indicator: @Composable (tabPositions: List<TabPosition>)  : @Composable {
}

fun PrimaryScrollableTabRow(selectedTabIndex: Int, modifier: Modifier, scrollState: ScrollState, containerColor: Color, contentColor: Color, edgePadding: Dp, indicator: @Composable (tabPositions: List<TabPosition>)  : @Composable {
}

fun SecondaryScrollableTabRow(selectedTabIndex: Int, modifier: Modifier, scrollState: ScrollState, containerColor: Color, contentColor: Color, edgePadding: Dp, indicator: @Composable (tabPositions: List<TabPosition>)  : @Composable {
}

fun ScrollableTabRow(selectedTabIndex: Int, modifier: Modifier, containerColor: Color, contentColor: Color, edgePadding: Dp, indicator: @Composable (tabPositions: List<TabPosition>)  : @Composable {
}

fun PrimaryIndicator(modifier: Modifier, width: Dp, height: Dp, color: Color, shape: Shape)  : @Composable {
}

fun SecondaryIndicator(modifier: Modifier, height: Dp, color: Color)  : @Composable {
}

fun colors()  : @Composable {
}

fun colors(activeContainerColor: Color, activeContentColor: Color, activeBorderColor: Color, inactiveContainerColor: Color, inactiveContentColor: Color, inactiveBorderColor: Color, disabledActiveContainerColor: Color, disabledActiveContentColor: Color, disabledActiveBorderColor: Color, disabledInactiveContainerColor: Color, disabledInactiveContentColor: Color, disabledInactiveBorderColor: Color)  : @Composable {
}

fun ActiveIcon()  : @Composable {
}

fun Icon(active: Boolean, activeContent: @Composable () -> Unit, inactiveContent: (@Composable () -> Unit))  : @Composable {
}

fun Icon(imageVector: ImageVector, contentDescription: String?, modifier: Modifier, tint: Color)  : @Composable {
}

fun Icon(bitmap: ImageBitmap, contentDescription: String?, modifier: Modifier, tint: Color)  : @Composable {
}

fun Icon(painter: Painter, contentDescription: String?, modifier: Modifier, tint: Color)  : @Composable {
}

fun TopAppBar(title: @Composable () -> Unit, modifier: Modifier, navigationIcon: @Composable () -> Unit, actions: @Composable RowScope.() -> Unit, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior)  : @Composable {
}

fun SmallTopAppBar(title: @Composable () -> Unit, modifier: Modifier, navigationIcon: @Composable () -> Unit, actions: @Composable RowScope.() -> Unit, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior)  : @Composable {
}

fun CenterAlignedTopAppBar(title: @Composable () -> Unit, modifier: Modifier, navigationIcon: @Composable () -> Unit, actions: @Composable RowScope.() -> Unit, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior)  : @Composable {
}

fun MediumTopAppBar(title: @Composable () -> Unit, modifier: Modifier, navigationIcon: @Composable () -> Unit, actions: @Composable RowScope.() -> Unit, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior)  : @Composable {
}

fun LargeTopAppBar(title: @Composable () -> Unit, modifier: Modifier, navigationIcon: @Composable () -> Unit, actions: @Composable RowScope.() -> Unit, windowInsets: WindowInsets, colors: TopAppBarColors, scrollBehavior: TopAppBarScrollBehavior)  : @Composable {
}

fun BottomAppBar(actions: @Composable RowScope.() -> Unit, modifier: Modifier, floatingActionButton: @Composable (() -> Unit), containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets)  : @Composable {
}

fun BottomAppBar(actions: @Composable RowScope.() -> Unit, modifier: Modifier, floatingActionButton: @Composable (() -> Unit), containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, scrollBehavior: BottomAppBarScrollBehavior)  : @Composable {
}

fun BottomAppBar(modifier: Modifier, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, content: @Composable RowScope.() -> Unit)  : @Composable {
}

fun BottomAppBar(modifier: Modifier, containerColor: Color, contentColor: Color, tonalElevation: Dp, contentPadding: PaddingValues, windowInsets: WindowInsets, scrollBehavior: BottomAppBarScrollBehavior, content: @Composable RowScope.() -> Unit)  : @Composable {
}

fun topAppBarColors()  : @Composable {
}

fun topAppBarColors(containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color)  : @Composable {
}

fun centerAlignedTopAppBarColors()  : @Composable {
}

fun centerAlignedTopAppBarColors(containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color)  : @Composable {
}

fun mediumTopAppBarColors()  : @Composable {
}

fun mediumTopAppBarColors(containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color)  : @Composable {
}

fun largeTopAppBarColors()  : @Composable {
}

fun largeTopAppBarColors(containerColor: Color, scrolledContainerColor: Color, navigationIconContentColor: Color, titleContentColor: Color, actionIconContentColor: Color)  : @Composable {
}

fun pinnedScrollBehavior(state: TopAppBarState, canScroll: () -> Boolean)  : @Composable {
}

fun enterAlwaysScrollBehavior(state: TopAppBarState, canScroll: () -> Boolean, snapAnimationSpec: AnimationSpec<Float>, flingAnimationSpec: DecayAnimationSpec<Float>)  : @Composable {
}

fun exitUntilCollapsedScrollBehavior(state: TopAppBarState, canScroll: () -> Boolean, snapAnimationSpec: AnimationSpec<Float>, flingAnimationSpec: DecayAnimationSpec<Float>)  : @Composable {
}

fun rememberTopAppBarState(initialHeightOffsetLimit: Float, initialHeightOffset: Float, initialContentOffset: Float)  : @Composable {
}

fun exitAlwaysScrollBehavior(state: BottomAppBarState, canScroll: () -> Boolean, snapAnimationSpec: AnimationSpec<Float>, flingAnimationSpec: DecayAnimationSpec<Float>)  : @Composable {
}

fun rememberBottomAppBarState(initialHeightOffsetLimit: Float, initialHeightOffset: Float, initialContentOffset: Float)  : @Composable {
}

fun OutlinedTextField(value: String, onValueChange: (String) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: @Composable (() -> Unit), placeholder: @Composable (() -> Unit), leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), prefix: @Composable (() -> Unit), suffix: @Composable (() -> Unit), supportingText: @Composable (() -> Unit), isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
}

fun OutlinedTextField(value: TextFieldValue, onValueChange: (TextFieldValue) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: @Composable (() -> Unit), placeholder: @Composable (() -> Unit), leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), prefix: @Composable (() -> Unit), suffix: @Composable (() -> Unit), supportingText: @Composable (() -> Unit), isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
}

fun OutlinedTextField(value: String, onValueChange: (String) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: @Composable (() -> Unit), placeholder: @Composable (() -> Unit), leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), supportingText: @Composable (() -> Unit), isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
}

fun OutlinedTextField(value: TextFieldValue, onValueChange: (TextFieldValue) -> Unit, modifier: Modifier, enabled: Boolean, readOnly: Boolean, textStyle: TextStyle, label: @Composable (() -> Unit), placeholder: @Composable (() -> Unit), leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), supportingText: @Composable (() -> Unit), isError: Boolean, visualTransformation: VisualTransformation, keyboardOptions: KeyboardOptions, keyboardActions: KeyboardActions, singleLine: Boolean, maxLines: Int, minLines: Int, interactionSource: MutableInteractionSource, shape: Shape, colors: TextFieldColors)  : @Composable {
}

fun Snackbar(modifier: Modifier, action: @Composable (() -> Unit), dismissAction: @Composable (() -> Unit), actionOnNewLine: Boolean, shape: Shape, containerColor: Color, contentColor: Color, actionContentColor: Color, dismissActionContentColor: Color, content: @Composable () -> Unit)  : @Composable {
}

fun Snackbar(snackbarData: SnackbarData, modifier: Modifier, actionOnNewLine: Boolean, shape: Shape, containerColor: Color, contentColor: Color, actionColor: Color, actionContentColor: Color, dismissActionContentColor: Color)  : @Composable {
}

fun ListItem(headlineContent: @Composable () -> Unit, modifier: Modifier, overlineContent: @Composable (() -> Unit), supportingContent: @Composable (() -> Unit), leadingContent: @Composable (() -> Unit), trailingContent: @Composable (() -> Unit), colors: ListItemColors, tonalElevation: Dp, shadowElevation: Dp)  : @Composable {
}

fun colors()  : @Composable {
}

fun RadioButton(selected: Boolean, onClick: (() -> Unit)?, modifier: Modifier, enabled: Boolean, colors: RadioButtonColors, interactionSource: MutableInteractionSource)  : @Composable {
}

fun colors()  : @Composable {
}

fun colors(selectedColor: Color, unselectedColor: Color, disabledSelectedColor: Color, disabledUnselectedColor: Color)  : @Composable {
}

fun BadgedBox(badge: @Composable BoxScope.() -> Unit, modifier: Modifier, content: @Composable BoxScope.() -> Unit)  : @Composable {
}

fun Badge(modifier: Modifier, containerColor: Color, contentColor: Color, content: @Composable (RowScope.() -> Unit))  : @Composable {
}

fun richTooltipColors()  : @Composable {
}

fun richTooltipColors(containerColor: Color, contentColor: Color, titleContentColor: Color, actionContentColor: Color)  : @Composable {
}

fun rememberPlainTooltipPositionProvider(spacingBetweenTooltipAndAnchor: Dp)  : @Composable {
}

fun rememberRichTooltipPositionProvider(spacingBetweenTooltipAndAnchor: Dp)  : @Composable {
}

fun AssistChip(onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource)  : @Composable {
}

fun AssistChip(onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource)  : @Composable {
}

fun ElevatedAssistChip(onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource)  : @Composable {
}

fun ElevatedAssistChip(onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource)  : @Composable {
}

fun FilterChip(selected: Boolean, onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), shape: Shape, colors: SelectableChipColors, elevation: SelectableChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource)  : @Composable {
}

fun ElevatedFilterChip(selected: Boolean, onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, leadingIcon: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), shape: Shape, colors: SelectableChipColors, elevation: SelectableChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource)  : @Composable {
}

fun InputChip(selected: Boolean, onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, leadingIcon: @Composable (() -> Unit), avatar: @Composable (() -> Unit), trailingIcon: @Composable (() -> Unit), shape: Shape, colors: SelectableChipColors, elevation: SelectableChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource)  : @Composable {
}

fun SuggestionChip(onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, icon: @Composable (() -> Unit), shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource)  : @Composable {
}

fun SuggestionChip(onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, icon: @Composable (() -> Unit), shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource)  : @Composable {
}

fun ElevatedSuggestionChip(onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, icon: @Composable (() -> Unit), shape: Shape, colors: ChipColors, elevation: ChipElevation, border: BorderStroke, interactionSource: MutableInteractionSource)  : @Composable {
}

fun ElevatedSuggestionChip(onClick: () -> Unit, label: @Composable () -> Unit, modifier: Modifier, enabled: Boolean, icon: @Composable (() -> Unit), shape: Shape, colors: ChipColors, elevation: ChipElevation, border: ChipBorder, interactionSource: MutableInteractionSource)  : @Composable {
}

fun assistChipColors()  : @Composable {
}

fun assistChipColors(containerColor: Color, labelColor: Color, leadingIconContentColor: Color, trailingIconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconContentColor: Color, disabledTrailingIconContentColor: Color)  : @Composable {
}

fun assistChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
}

fun elevatedAssistChipColors()  : @Composable {
}

fun elevatedAssistChipColors(containerColor: Color, labelColor: Color, leadingIconContentColor: Color, trailingIconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconContentColor: Color, disabledTrailingIconContentColor: Color)  : @Composable {
}

fun elevatedAssistChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
}

fun filterChipColors()  : @Composable {
}

fun filterChipColors(containerColor: Color, labelColor: Color, iconColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, selectedContainerColor: Color, disabledSelectedContainerColor: Color, selectedLabelColor: Color, selectedLeadingIconColor: Color, selectedTrailingIconColor: Color)  : @Composable {
}

fun filterChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
}

fun elevatedFilterChipColors()  : @Composable {
}

fun elevatedFilterChipColors(containerColor: Color, labelColor: Color, iconColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, selectedContainerColor: Color, disabledSelectedContainerColor: Color, selectedLabelColor: Color, selectedLeadingIconColor: Color, selectedTrailingIconColor: Color)  : @Composable {
}

fun elevatedFilterChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
}

fun inputChipColors()  : @Composable {
}

fun inputChipColors(containerColor: Color, labelColor: Color, leadingIconColor: Color, trailingIconColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledLeadingIconColor: Color, disabledTrailingIconColor: Color, selectedContainerColor: Color, disabledSelectedContainerColor: Color, selectedLabelColor: Color, selectedLeadingIconColor: Color, selectedTrailingIconColor: Color)  : @Composable {
}

fun inputChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
}

fun suggestionChipColors()  : @Composable {
}

fun suggestionChipColors(containerColor: Color, labelColor: Color, iconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledIconContentColor: Color)  : @Composable {
}

fun suggestionChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
}

fun elevatedSuggestionChipColors()  : @Composable {
}

fun elevatedSuggestionChipColors(containerColor: Color, labelColor: Color, iconContentColor: Color, disabledContainerColor: Color, disabledLabelColor: Color, disabledIconContentColor: Color)  : @Composable {
}

fun elevatedSuggestionChipElevation(elevation: Dp, pressedElevation: Dp, focusedElevation: Dp, hoveredElevation: Dp, draggedElevation: Dp, disabledElevation: Dp)  : @Composable {
}

fun DateRangePicker(state: DateRangePickerState, modifier: Modifier, dateFormatter: DatePickerFormatter, title: (@Composable () -> Unit)  : @Composable {
}

fun DateRangePickerTitle(displayMode: DisplayMode, modifier: Modifier)  : @Composable {
}

fun SnackbarHost(hostState: SnackbarHostState, modifier: Modifier, snackbar: @Composable (SnackbarData) -> Unit)  : @Composable {
}

fun LinearProgressIndicator(progress: () -> Float, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap)  : @Composable {
}

fun LinearProgressIndicator(progress: () -> Float, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap, gapSize: Dp, drawStopIndicator: (DrawScope.() -> Unit)  : @Composable {
}

fun LinearProgressIndicator(modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap)  : @Composable {
}

fun LinearProgressIndicator(modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap, gapSize: Dp)  : @Composable {
}

fun LinearProgressIndicator(progress: Float, modifier: Modifier, color: Color, trackColor: Color, strokeCap: StrokeCap)  : @Composable {
}

fun LinearProgressIndicator(progress: Float, modifier: Modifier, color: Color, trackColor: Color)  : @Composable {
}

fun LinearProgressIndicator(modifier: Modifier, color: Color, trackColor: Color)  : @Composable {
}

fun CircularProgressIndicator(progress: () -> Float, modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap)  : @Composable {
}

fun CircularProgressIndicator(progress: () -> Float, modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap, gapSize: Dp)  : @Composable {
}

fun CircularProgressIndicator(modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap)  : @Composable {
}

fun CircularProgressIndicator(progress: Float, modifier: Modifier, color: Color, strokeWidth: Dp, trackColor: Color, strokeCap: StrokeCap)  : @Composable {
}

fun CircularProgressIndicator(progress: Float, modifier: Modifier, color: Color, strokeWidth: Dp)  : @Composable {
}

fun CircularProgressIndicator(modifier: Modifier, color: Color, strokeWidth: Dp)  : @Composable {
}

fun HorizontalDivider(modifier: Modifier, thickness: Dp, color: Color)  : @Composable {
}

fun VerticalDivider(modifier: Modifier, thickness: Dp, color: Color)  : @Composable {
}

fun Divider(modifier: Modifier, thickness: Dp, color: Color)  : @Composable {
}

fun Indicator(state: PullToRefreshState, modifier: Modifier, color: Color)  : @Composable {
}
