import React, { useEffect, useState } from 'react';
import logo from './logo.svg';
import './App.css';


interface result  {
  name:String,
  bitcoin_height:number,
  timestamp:String
}

function App() {
  const[data, setData] = useState<result>();

  const getData = async() => {
     const response = await fetch('http://localhost:3000/get_bitcoin', {
          method:'GET',
          headers:{
            "Content-Type":"application/json"
          },
          
     }) 
     const json = await response.json() 
     
     if(response.ok) {
      let time = new Date(json.timestamp * 1000)
      let res = {
        name: json.name,
        bitcoin_height: json.bitcoin_height,
        timestamp: time.toString()
       }
      setData(res);}
  }

  useEffect(() => {
    getData();
   
  }, [])

  return (
    console.log(data),
    <div className="App">
      {
        data && (
          <div>
          <p>Name : {data.name}</p>
          <p>Height : {data.bitcoin_height}</p>
          <p>Timestamp : {data.timestamp}</p>
          </div>
        )
        
      }
    </div>
  );
}

export default App;
