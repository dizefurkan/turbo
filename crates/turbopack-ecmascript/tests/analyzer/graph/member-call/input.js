let array = [1, 2, 3];
let item = array[1];
let concatenated_array = array.concat(4, 5, 6, [7, 8, 9], `hello ${item}`);
let pick_array1 = [[1, 2, 3]][0].concat([4, 5, 6, unknown]);
let pick_array2 = [[1, 2, 3]][0].concat(4, 5, 6, unknown);
let arrays = [1, 2, 3];
arrays = [4, 5, 6];
let item_options = arrays[1];
let concatenated_array_options = array.concat([7, 8, 9]);