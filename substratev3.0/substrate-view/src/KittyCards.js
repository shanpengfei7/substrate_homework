import React from 'react'
import { Button, Card, Grid, Message, Modal, Form, Label } from 'semantic-ui-react'

import KittyAvatar from './KittyAvatar'
import { TxButton } from './substrate-lib/components'

// --- About Modal ---

const TransferModal = props => {
  const { kitty, accountPair, setStatus } = props
  const [open, setOpen] = React.useState(false)
  const [formValue, setFormValue] = React.useState({})

  const formChange = key => (ev, el) => {
    setFormValue({ ...formValue, [key]: el.value })
  }

  const confirmAndClose = (unsub) => {
    unsub()
    setOpen(false)
  }

  return <Modal onClose={() => setOpen(false)} onOpen={() => setOpen(true)} open={open}
    trigger={<Button basic color='blue'>转让</Button>}>
    <Modal.Header>毛孩转让</Modal.Header>
    <Modal.Content><Form>
      <Form.Input fluid label='毛孩 ID' readOnly value={kitty.id}/>
      <Form.Input fluid label='转让对象' placeholder='对方地址' onChange={formChange('target')}/>
    </Form></Modal.Content>
    <Modal.Actions>
      <Button basic color='grey' onClick={() => setOpen(false)}>取消</Button>
      <TxButton
        accountPair={accountPair} label='确认转让' type='SIGNED-TX' setStatus={setStatus}
        onClick={confirmAndClose}
        attrs={{
          palletRpc: 'kittiesModule',
          callable: 'transfer',
          inputParams: [formValue.target, kitty.id],
          paramFields: [true, true]
        }}
      />
    </Modal.Actions>
  </Modal>
}

const MarketModal = props => {
  const { kitty, accountPair, setStatus } = props
  const [open, setOpen] = React.useState(false)
  const [markets, setMarkets] = React.useState({})

  const marketChange = key => (ev, el) => {
    setMarkets({ ...markets, [key]: el.value })
  }

  const confirmAndClose = (unsub) => {
    unsub()
    setOpen(false)
  }

  return <Modal onClose={() => setOpen(false)} onOpen={() => setOpen(true)} open={open}
    trigger={<Button basic color='blue'>投入市场</Button>}>
    <Modal.Header>毛孩投入市场</Modal.Header>
    <Modal.Content><Form>
      <Form.Input fluid label='毛孩 ID' readOnly value={kitty.id}/>
      <Form.Input fluid label='毛孩价格' placeholder='毛孩价格' type='number' onChange={marketChange('price')}/>
    </Form></Modal.Content>
    <Modal.Actions>
      <Button basic color='grey' onClick={() => setOpen(false)}>取消</Button>
      <TxButton
        accountPair={accountPair} label='确认投入市场' type='SIGNED-TX' setStatus={setStatus}
        onClick={confirmAndClose}
        attrs={{
          palletRpc: 'kittiesModule',
          callable: 'market',
          inputParams: [kitty.id, markets.price],
          paramFields: [true, true]
        }}
      />
    </Modal.Actions>
  </Modal>
}

const BuyModal = props => {
  const { kitty, accountPair, setStatus } = props
  const [open, setOpen] = React.useState(false)

  const confirmAndClose = (unsub) => {
    unsub()
    setOpen(false)
  }

  return <Modal onClose={() => setOpen(false)} onOpen={() => setOpen(true)} open={open}
    trigger={<Button basic color='blue'>购买</Button>}>
    <Modal.Header>毛孩购买</Modal.Header>
    <Modal.Content><Form>
      <Form.Input fluid label='毛孩 ID' readOnly value={kitty.id}/>
      <Form.Input fluid label='毛孩价格' readOnly value={kitty.market}/>
    </Form></Modal.Content>
    <Modal.Actions>
      <Button basic color='grey' onClick={() => setOpen(false)}>取消</Button>
      <TxButton
        accountPair={accountPair} label='确认购买' type='SIGNED-TX' setStatus={setStatus}
        onClick={confirmAndClose}
        attrs={{
          palletRpc: 'kittiesModule',
          callable: 'buy',
          inputParams: [kitty.id, kitty.market],
          paramFields: [true, true]
        }}
      />
    </Modal.Actions>
  </Modal>
}

// --- About Kitty Card ---

const KittyCard = props => {
  const { kitty, accountPair, setStatus } = props
  const { id = null, dna = null, owner = null, market = null } = kitty
  const displayDna = dna && dna.join(', ')
  const displayMarket = market || '未投入市场'
  const displayId = id === null ? '' : (id < 10 ? `0${id}` : id.toString())
  const isSelf = accountPair.address === kitty.owner

  console.log(kitty);

  return <Card>
    { isSelf && <Label as='a' floating color='teal'>我的</Label> }
    <KittyAvatar dna={dna} />
    <Card.Content>
      <Card.Header>ID 号: {displayId}</Card.Header>
      <Card.Meta style={{ overflowWrap: 'break-word' }}>
        基因: <br/>
        {displayDna}
      </Card.Meta>
      <Card.Description>
        <p style={{ overflowWrap: 'break-word' }}>
          猫奴:<br/>
          {owner}
        </p>
        <p style={{ overflowWrap: 'break-word' }}>
          市场:<br/>
          {displayMarket}
        </p>
      </Card.Description>
    </Card.Content>
    <Card.Content extra style={{ textAlign: 'center' }}>{ owner === accountPair.address
      ? <TransferModal kitty={kitty} accountPair={accountPair} setStatus={setStatus}/>
      : ''
    }</Card.Content>
    <Card.Content extra style={{ textAlign: 'center' }}>{ owner === accountPair.address && !kitty.market
      ? <MarketModal kitty={kitty} accountPair={accountPair} setStatus={setStatus}/>
      : ''
    }</Card.Content>
    <Card.Content extra style={{ textAlign: 'center' }}>{ owner !== accountPair.address && kitty.market
      ? <BuyModal kitty={kitty} accountPair={accountPair} setStatus={setStatus}/>
      : ''
    }</Card.Content>
  </Card>
}

const KittyCards = props => {
  const { kitties, accountPair, setStatus } = props

  if (kitties.length === 0) {
    return <Message info>
      <Message.Header>现在连一只毛孩都木有，赶快创建一只&nbsp;
        <span role='img' aria-label='point-down'>👇</span>
      </Message.Header>
    </Message>
  }

  return <Grid columns={3}>{kitties.map((kitty, i) =>
    <Grid.Column key={`kitty-${i}`}>
      <KittyCard kitty={kitty} accountPair={accountPair} setStatus={setStatus}/>
    </Grid.Column>
  )}</Grid>
}

export default KittyCards
