import { NowRequest, NowResponse } from '@now/node'
import fetch from 'node-fetch'

export default async (
  request: NowRequest,
  response: NowResponse
): Promise<void> => {
  const payload = JSON.parse(request.body.payload)

  const {
    type,
    channel,
    callback_id: callbackId,
    action_ts: actionTs,
    submission,
    response_url: responseUrl,
    state
  } = payload as {
    type: 'dialog_submission'
    channel: { id: string }
    callback_id: string
    action_ts: string
    submission: { trackNumber: string }
    response_url: string
    state: string
  }

  if (type !== 'dialog_submission') {
    response.status(200).send('')
    return
  }

  if (callbackId !== 'confirm ecog--reset-tt-by-track-number') {
    response.status(200).send('')
    return
  }

  if (submission.trackNumber !== state) {
    response.status(200).json({
      errors: [
        {
          name: 'trackNumber',
          error: '番号が一致していません'
        }
      ]
    })

    return
  }

  await fetch(
    'https://api.github.com/repos/geekcojp/ecogreen--slack/dispatches',
    {
      method: 'POST',
      headers: {
        accept: 'application/vnd.github.v3+json',
        authorization: `Bearer 045997ee0e577fb11f3849c4af23b41695009b89`
      },
      body: JSON.stringify({
        event_type: 'repository_dispatch',
        client_payload: {
          channel_id: channel.id,
          slack_token:
            'xoxp-2659222021-179415996097-372606790839-611595f547fae0cd6f34be2666e0e333',
          response_url: responseUrl,
          ts: actionTs,
          track_number: submission.trackNumber
        }
      })
    }
  )
    .then((res) => {
      console.log('end', res)
      response.status(200).end()
    })
    .catch((err) => {
      // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
      console.log(`Error: ${err}`)
      // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
      response.status(200).end()
    })
}
