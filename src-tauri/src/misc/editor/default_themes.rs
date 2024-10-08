pub(crate) fn default() -> String {
    r#"<!-- You can use the most common bindings on this editor -->
<!-- As an example: "CRTL + /" creates this comment line -->
<!-- You can use common CSS inline styling OR Tailwindcss, it's up to you. -->
<!-- For Tailwind, refer to this documentation: https://tailwindcss.com/docs -->
<!-- Slashes as comments will get rendered since this is an HTML editor -->

<!-- If you want your messages to be deleted and do not want to handle it on this editor, PLEASE do not forget to set the main div id -->
<!-- If you do not handle it yourself and do not set it, the app WILL NOT remove old messages for you. -->
<div id={id} class="w-full h-full">
    <!--
    Outer container that takes up the full width and height of its parent.
    This ensures that all the content inside has the necessary space to render fully.
    -->
    <div class="flex flex-col items-start justify-center bg-transparent m-2 text-white max-w-[600px]">
        <!--
        Inner flex container holding the platform badge, username, and message.
        The 'flex-col' layout makes the inner elements stack vertically.
        The 'items-start' aligns all the items at the top of this container, preventing vertical misalignment.
        The 'max-w-[600px]' ensures that this container does not exceed 600px in width, preventing overflow.
        -->
        <div class="flex flex-row items-start w-full">
            <!--
            Flex container for the platform and user badges.
            'flex-row' arranges the badges horizontally.
            'items-start' ensures these badges align at the top, preventing them from stretching vertically.
            -->
            <div class="flex flex-row items-start space-x-2 mr-2">
                <!-- Placeholder for platform-specific badge (e.g., Twitch, YouTube) -->
                {platform}
                <!-- Displays user badges next to the platform badge -->
                <!-- There is also: {badge_1} to {badge_3}. You can also use {formatedBadges (*WIP)} to set automatically badges. -->
                {badges}
            </div>

            <!--
            Flex container for the username and message.
            'flex-row' arranges the username and message side by side.
            'items-start' ensures that both the username and message align at the top, which is crucial for keeping the username from being vertically centered when the message is long.
            'space-x-0.5' adds a small space between the username and the message.
            -->
            <div class="flex flex-row items-start space-x-0.5 w-full break-words overflow-hidden text-ellipsis space-x-2">
                <!--
                The username is styled with 'flex-none' to prevent it from stretching.
                This ensures the username stays fixed in size, maintaining its position at the top left of the container, regardless of the message length.
                The 'color' style is dynamically set based on user preference or platform data.
                -->
                <p style="color: {color};" class="flex-none">{user}</p>:

                <!--
                 The message text is placed in a flexible container ('flex-1') that allows it to take up the remaining space next to the username.
                'break-words' ensures that words break to the next line if they exceed the container width, preventing horizontal scrolling or overflow.
                'overflow-hidden' prevents any text that exceeds the container from spilling out, ensuring a clean layout.
                'whitespace-pre-wrap' preserves whitespace in the message text and wraps lines as necessary, ensuring the message is displayed as entered, while respecting the container boundaries.
                'text-ellipsis' adds an ellipsis (...) if the text overflows, but since 'break-words' and 'overflow-hidden' are in place, this is more of a fallback measure.
                -->
                <p class="flex-1 break-words overflow-hidden whitespace-pre-wrap">{formatedMessage}</p>
            </div>
        </div>
    </div>
</div>

<!-- Imagination is your limit, do whatever you want. -->
<!-- After you're done and like what you're seeing, click "Save" to save the theme -->
<!-- if you don't, you'll lose everything. (PS: AutoSaving is planned) -->
<!-- If you're having problems with line breaking messages, please use this example to format your styling accordingly. -->
<!--
    ### Detailed Explanation of Word Breaking and Overflow Handling:
    1. **`break-words`**:
       - **Purpose**: Forces the text to break onto a new line if a word is too long to fit within the container.
       - **Why It’s Important**: Without this, long words (or strings with no spaces, like URLs) might overflow out of the container, causing layout issues. By enabling word breaking, we ensure that all content stays within its designated area.

    2. **`overflow-hidden`**:
       - **Purpose**: Ensures that any content that exceeds the container boundaries is hidden.
       - **Why It’s Important**: This prevents text or other content from spilling out of its container, which can disrupt the layout and create a poor user experience. In combination with `break-words`, this ensures that content stays visually contained within its intended area.

    3. **`whitespace-pre-wrap`**:
       - **Purpose**: Preserves the whitespace in the message as it was entered, and wraps text as necessary.
       - **Why It’s Important**: This setting respects the original formatting of the message (including spaces and line breaks) while ensuring the text wraps properly within the container. It’s particularly useful for maintaining the readability of user-generated content.

    4. **`text-ellipsis`**:
       - **Purpose**: Adds an ellipsis (`...`) if the content is too long to fit within the container.
       - **Why It’s Important**: Although `break-words` and `overflow-hidden` manage most of the overflow issues, `text-ellipsis` acts as a safety net, ensuring that if any text does somehow exceed its container, it’s truncated gracefully with an ellipsis.
-->"#.to_string()
}

pub(crate) fn sakura() -> String {
    r#"<div id={id} class="flex flex-col items-start max-w-xs font-sans p-2">
    <div class="flex items-center mb-2 ml-2">
        <div class="flex items-center bg-gradient-to-r from-pink-200 to-pink-300 px-3 py-1 rounded-full shadow-sm">
            <span class="text-sm font-bold text-pink-700 tracking-wide">{user}</span>
        </div>
        <div class="flex ml-2 space-x-1">
            {platform}
            {badges}
        </div>
    </div>
    <div class="relative bg-gradient-to-br from-pink-100 to-pink-200 text-gray-800 px-4 py-3 rounded-2xl border-2 border-pink-300 shadow-md min-w-[300px] max-w-[600px]">
        <div class="absolute left-4 top-0 transform -translate-y-1/2 rotate-45 w-3 h-3 bg-pink-100 border-l-2 border-t-2 border-pink-300"></div>
        <p class="text-sm leading-snug">
            {formatedMessage}
        </p>
        <div class="absolute -bottom-2 -right-2 text-pink-400">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"
                 stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                 class="lucide lucide-cherry">
                <path d="M2 17a5 5 0 0 0 10 0c0-2.76-2.5-5-5-3-2.5-2-5 .24-5 3Z"/>
                <path d="M12 17a5 5 0 0 0 10 0c0-2.76-2.5-5-5-3-2.5-2-5 .24-5 3Z"/>
                <path d="M7 14c3.22-2.91 4.29-8.75 5-12 1.66 2.38 4.94 9 5 12"/>
                <path d="M22 9c-4.29 0-7.14-2.33-10-7 5.71 0 10 4.67 10 7Z"/>
            </svg>
        </div>
    </div>
    <div class="text-xs text-gray-500 mt-1 ml-4">
        2:30 PM
    </div>
</div>"#.to_string()
}