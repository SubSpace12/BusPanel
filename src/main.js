const { invoke } = window.__TAURI__.tauri;



async function do_thing(id, number) {
  let busses = await invoke("get_bus_json", {lineId: id, lineNumber: number});
  return busses;
}


async function greet(designatedField, arr) {
  var startTime = performance.now();
  console.log(typeof arr);
  let to_display = "";
  let busses = await invoke("get_bus_times", {busArr: arr});
  for (let x = 0; x<busses.length; x++) {
    to_display += busses[x];
    to_display += '<br>';
  }
  designatedField.innerHTML = to_display;
  var endTime = performance.now();
  console.log("Function called!");
  console.log(endTime-startTime);
}

const a1 = do_thing("2143", "01");
const a2 = do_thing("2143", "02");
const a3 = do_thing("2098", "01");
const a4 = do_thing("2098", "02");


function doEveryMinute() {
  let af1 = document.getElementById("a1");
  let af2 = document.getElementById("a2");
  let mi1 = document.getElementById("m1");
  let mi2 = document.getElementById("m2");
  greet(af1, a1);
  greet(af2, a2);
  greet(mi1, a3);
  greet(mi2, a4);
}

window.onload = () => {
console.log(a1);
console.log("page loaded!");
doEveryMinute();
setInterval(doEveryMinute, 60*1000);
};
