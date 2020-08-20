export function init(tagId = 'desc') {
  return new Promise((resolve) => {
    const { appVersion, userAgent } = navigator;
  
    let obj = { appVersion, userAgent };
  
    let res = Object.entries(obj).reduce((str, [key, value]) => {
      str += `<div>
            <p>
              <span>${key}</span> : <span>${value}</span>
            </p>
          </div>`;
      return str;
    });
  
    document.querySelector(`#${tagId}`).innerHTML = res;
    setTimeout(() => {
      resolve()
    }, 1000)
  })
}

export function clear(tag = 'tbody') {
  document.querySelector(tag).innerText = '';
}

export function updateDesc(text, tag = 'message') {
  document.querySelector(`#${tag}`).innerText = text;
}

export function appendRow(res, tag = 'tbody') {
  let body = document.querySelector(tag);

  const { name, len, result, timing, count } = res;

  let row = `
    <tr>
      <td>${name}</td>
      <td>${len}</td>
      <td>${count || 1}</td>
      <td>${timing[0]}</td>
      <td>${timing[1]}</td>
    </tr>
  `;

  body.innerHTML = body.innerHTML + row;
}