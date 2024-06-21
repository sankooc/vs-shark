(function () {
    const ele = document.getElementById("files");
    ele.addEventListener('change', function () {
      var reader = new FileReader();
      reader.onload = function () {
        const arrayBuffer = this.result;
        const array = new Uint8Array(arrayBuffer);
        const iFrame = document.getElementById('iframe');
        console.log('size', array.length)
        iFrame.contentWindow.postMessage({type: 'init', body: array}, '*');
      };
      reader.readAsArrayBuffer(this.files[0]);
    }, false);
  }());