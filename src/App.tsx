import { useState, useEffect } from 'react';
import './App.css';
import init, {query_wasm} from 'wasm';
import JsonEditor from './JsonEditor'

function App() {
  const [ans, setAns] = useState({text:""});
  const full = `{
  "data": [
    {
      "content": {
        "labels": [
          {
            "text": "tempor labore",
            "type": 3
          }
        ],
        "level": 3
      },
      "id": 183,
      "name": "Celia",
      "value": 200
    },
    {
      "content": {
        "labels": [
          {
            "text": "proident tempor",
            "type": 1
          },
          {
            "text": "nisi commodo",
            "type": 2
          }
        ],
        "level": 1
      },
      "id": 148,
      "name": "Winters",
      "value": 1000
    },
    {
      "content": {
        "labels": [
          {
            "text": "nostrud proident",
            "type": 2
          },
          {
            "text": "ipsum ea",
            "type": 1
          }
        ],
        "level": 1
      },
      "id": 134,
      "name": "Brenda",
      "value": 5678
    },
    {
      "content": {
        "labels": [],
        "level": 3
      },
      "id": 144,
      "name": "Cathy",
      "value": 1234
    }
  ]
}`;

  const select = `{
  "data": {
    "id": true,
    "content": {
      "labels": true
    }
  }
}`;

  const [fullContent, setFullContent] = useState({
    text: full
  })
  const [selectContent, setSelectContent] = useState({
    text: select
  })

  useEffect(() => {
    init().then(() => {
      setAns({text: query_wasm(fullContent.text, selectContent.text)});
    })
  }, [fullContent, selectContent])
  return (
    <div className="App">
      <div className="left">
        <h2>Origin Json</h2>
        <div id="full" className="editor">
          <JsonEditor
            content={fullContent}
            readOnly={false}
            onChange={setFullContent}
            mode={"text"}
            />
        </div>
      </div>
      <div className="right">
        <h2>Query</h2>
        <div id="select" className="editor">
          <JsonEditor
            content={selectContent}
            readOnly={false}
            onChange={setSelectContent}
            mode={"text"}
            />
        </div>
        <h2>Query Result</h2>
        <div id="result" className="editor">
          <JsonEditor
            content={ans}
            readOnly={true}
            />
        </div>
      </div>
    </div>
  );
}

export default App;
