<template>
  <div class="absolute top-[5.2rem] min-w-full">
    <div class="mx-4 max-w-full space-y-4 rounded-lg bg-white bg-gray-800 p-5 shadow dark:bg-gray-700">
      <h1 class="text-2xl font-bold text-white">Help</h1>
      <div>
        <h2 class="text-lg font-bold text-white">Description</h2>
        <div class="text-gray-400 dark:text-gray-300">
          Simple calculator that also supports chaining of operations and dark mode
        </div>
      </div>
      <div>
        <h2 class="text-lg font-bold text-white">Shortcuts</h2>
        <div class="text-gray-400 dark:text-gray-300"><span class="font-bold">a</span> - Absolute of a number</div>
        <div class="text-gray-400 dark:text-gray-300"><span class="font-bold">f</span> - Factorial</div>
        <div class="text-gray-400 dark:text-gray-300"><span class="font-bold">p</span> - nth Power</div>
        <div class="text-gray-400 dark:text-gray-300"><span class="font-bold">r</span> - nth Root</div>
        <div class="text-gray-400 dark:text-gray-300"><span class="font-bold">c</span> - Clear</div>
        <div class="text-gray-400 dark:text-gray-300">
          The rest of digits and numeric operations are same as on keyboard.
        </div>
      </div>
      <div>
        <h2 class="text-lg font-bold text-white">Operations</h2>
        <div class="text-gray-400 dark:text-gray-300">
          <span class="font-bold">ABS</span> - Returns absolute value of a number
        </div>
        <div class="text-gray-400 dark:text-gray-300">
          <span class="font-bold">!</span> - Returns factorial of a number
        </div>
        <div class="text-gray-400 dark:text-gray-300">
          <span class="font-bold">xⁿ</span> - Returns nth power of a number
        </div>
        <div class="text-gray-400 dark:text-gray-300">
          <span class="font-bold">√x</span> - Returns nth root of a number. Second number is one to get root of
        </div>
      </div>
    </div>
  </div>
</template>
