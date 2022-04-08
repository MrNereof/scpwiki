import * as React from 'react';
import { Component } from 'react';
import Loader from "../util/loader";
import styled from "styled-components";
import WikidotModal from "../util/wikidot-modal";
import sleep from "../util/async-sleep";
import {wFetch} from "../util/fetch-util";


interface Props {
    pageId: string
    isNew?: boolean
    onCancel?: () => void
}

interface State {
    title: string
    source: string
    loading: boolean
    saving: boolean
    savingSuccess?: boolean
    error?: string
}


function guessTitle(pageId) {
    const pageIdSplit = pageId.split(':', 1);
    if (pageIdSplit.length === 2)
        pageId = pageIdSplit[1];
    else pageId = pageIdSplit[0];

    let result = '';
    let brk = true;
    for (let i = 0; i < pageId.length; i++) {
        let char = pageId[i];
        if (char === '-') {
            if (!brk) {
                result += ' ';
            }
            brk = true;
            continue;
        }
        if (brk) {
            char = char.toUpperCase();
            brk = false;
        } else {
            char = char.toLowerCase();
        }
        result += char;
    }
    return result;
}


const Styles = styled.div`
.editor-area {
  position: relative;
  
  &.loading {
    &::after {
      content: ' ';
      position: absolute;
      background: #0000003f;
      z-index: 0;
      left: 0;
      right: 0;
      top: 0;
      bottom: 0;
    }
    .loader {
      position: absolute;
      left: 16px;
      top: 16px;
      z-index: 1;
    }
  }
}
`;


class ArticleEditor extends Component<Props, State> {
    constructor(props) {
        super(props);
        this.state = {
            title: guessTitle(props.pageId),
            source: '',
            loading: true,
            saving: false
        }
    }

    componentDidMount() {
        if (!this.props.isNew) {
            // load existing source. not yet
        } else {
            this.setState({ loading: false })
        }
    }

    onSubmit = async () => {
        this.setState({ saving: true, error: null, savingSuccess: false });
        const input = {
            pageId: this.props.pageId,
            source: this.state.source
        };
        try {
            await wFetch(`/api/articles/new`, {method: 'POST', sendJson: true, body: input});
            this.setState({ saving: false, savingSuccess: true });
            await sleep(2000);
            this.setState({ savingSuccess: false });
            window.location.href = `/${this.props.pageId}`;
        } catch (e) {
            this.setState({ saving: false, error: e.error || 'Ошибка связи с сервером' });
        }
    };

    onCancel = () => {
        if (this.props.onCancel)
            this.props.onCancel()
    };

    onChange = (e) => {
        // @ts-ignore
        this.setState({[e.target.name]: e.target.value})
    };

    onCloseError = () => {
        this.setState({error: null});
    };

    render() {
        const { title, source, loading, saving, savingSuccess, error } = this.state;
        return (
            <Styles>
                { saving && <WikidotModal isLoading>Сохранение...</WikidotModal> }
                { savingSuccess && <WikidotModal>Успешно сохранено!</WikidotModal> }
                { error && (
                    <WikidotModal buttons={[{title: 'Закрыть', onClick: this.onCloseError}]}>
                        <strong>Ошибка:</strong> {error}
                    </WikidotModal>
                ) }
                <h1>Создать страницу</h1>
                <form id="edit-page-form" onSubmit={this.onSubmit}>
                    <table className="form" style={{ margin: '0.5em auto 1em 0' }}>
                        <tbody>
                        <tr>
                            <td>Заголовок страницы:</td>
                            <td>
                                <input id="edit-page-title" value={title} onChange={this.onChange} name="title" type="text" size={35} maxLength={128} style={{ fontWeight: 'bold', fontSize: '130%' }} />
                            </td>
                        </tr>
                        </tbody>
                    </table>
                    <div className={`editor-area ${loading?'loading':''}`}>
                        <textarea id="edit-page-textarea" value={source} onChange={this.onChange} name="source" rows={20} cols={60} style={{ width: '95%' }} disabled={loading} />
                        { loading && <Loader className="loader" /> }
                    </div>
                    <div className="buttons alignleft">
                        <input id="edit-cancel-button" className="btn btn-danger" type="button" name="cancel" value="Отмена" onClick={this.onCancel} disabled={loading} />
                        <input id="edit-save-button" className="btn btn-primary" type="button" name="save" value="Сохранить" onClick={this.onSubmit} disabled={loading} />
                    </div>
                </form>
            </Styles>
        )
    }
}


export default ArticleEditor