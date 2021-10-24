import React, { Component, useState, useEffect } from "react";
import { Container, Col, Row } from "react-bootstrap";
import { Chart } from "react-google-charts";

export default class Runtime extends Component { 
    constructor () {
        super();
        //this.state = ({counter:1,data: [["time","Ethereum","Salana","Local","Cloudfare","Polka"],[0,0,0,0,0,0]]});
        this.state = ({counter:1,data: [[["time","Solana"],[0,0]],[["time","Local"],[0,0]],[["time","Cloudflare"],[0,0]],[["time","Polkadot"],[0,0]],[["time","Ethereum"],[0,0]]]});
        this.reload = () => {
          fetch("http://localhost:8000/records/"+this.state.counter+"?js="+
          this.state.data.map(e => e.length === 1 ? 0 : e[e.length-1][0]).join(","),
          {method:"get"}).then(res => res.json())
          .then( (res) => {
            console.log(res);
            var d = this.state.data;
            for (var i = 0; i < res.data.length; i ++) {
              for (var j = 0; j < res.data[i].length; j++) {
                d[i].push(res.data[i][j]);
              }
            }
            this.setState({data: d});
          })
          this.setState({counter:this.state.counter+1})
        
          if (this.state.counter < 120) setTimeout(this.reload,1000);
        }
    }

    
    componentDidMount() {
      this.reload()
        
    }
    render() {
      return <Container><Row>{this.state.data.map((arr) => {
      return <Col><Container><Chart
      width={550}
      height={500}
      chartType="LineChart"
      loader={<div>Loading Chart</div>}
      data={arr}

      options={{
        title: 'Simulated Completion Times for ' + arr[0][1],
         hAxis: {
           minValue: 0,
        title: 'Execution #',
      },
      legend: 'none',
      
      vAxis: {
        viewWindow: {
          min:0
        },
        title: 'Time of execution (ms)',
      }}}
    /></Container></Col>;})
    }
    </Row></Container>
  }

}
