import React, { Component, useState, useEffect } from "react";
import { Form, Row, Col, Container, Button} from 'react-bootstrap';

import { withRouter } from 'react-router-dom'
// this also works with react-router-native

const Btn = withRouter(({ history }) => (
  <Button
  className="mb-3"
    type='submit'
    onClick={() => { history.push('/runtime') }}
  >
    Run
  </Button>
))

export default class NewProcess extends Component {
    render() {
      return <Container><Row style={{marginTop: 20 + 'px'}}><h1>Enter Clockchain Code</h1></Row><Form>
        <Row className="mb-3">
      <Form.Group className="mb-3" id="formGridCheckbox">
    <Form.Check type="checkbox" name = "eth" label="Ethereum" checked />
    <Form.Check type="checkbox" name = "sol" label="Solana" checked/>
    <Form.Check type="checkbox" name = "clf" label="Local" checked/>
    <Form.Check type="checkbox" name = "clf" label="Polkadot" checked/>
    <Form.Check type="checkbox" name = "clf" label="Cloudfare" checked/>
  </Form.Group>
  </Row>
  <Row className="mb-3">
      <Form.Group className="mb-3" controlId="exampleForm.ControlTextarea1">
        <Form.Label>Code</Form.Label>
        <Form.Control name = "code" as="textarea" rows={20} />
      </Form.Group>
      </Row>
      <Row>
      <Form.Group>
        <Btn/>
      </Form.Group>
      </Row>
    </Form></Container>;
    }
  }

