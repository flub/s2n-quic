import { useState, useMemo, default as React } from "react";
import { useLocation } from "react-router-dom";
import { makeStyles, withStyles } from "@material-ui/core/styles";
import Box from "@material-ui/core/Box";
import List from "@material-ui/core/List";
import Link from "@material-ui/core/Link";
import ListItem from "@material-ui/core/ListItem";
import ListItemText from "@material-ui/core/ListItemText";
import Paper from "@material-ui/core/Paper";
import Dialog from "@material-ui/core/Dialog";
import Tooltip from "@material-ui/core/Tooltip";
import Chip from "@material-ui/core/Chip";
import FileCopyIcon from "@material-ui/icons/FileCopy";
import Typography from "@material-ui/core/Typography";
import clsx from "clsx";
import copyToClipboard from "copy-to-clipboard";
import { Requirements } from "./spec";

export function Section({ spec, section }) {
  const requirements = section.requirements || [];
  // the datagrid is crashing on link transition and needs to be rebuilt
  const key = `${spec.id}--${section.id}`;
  return (
    <>
      <h2>
        {section.id} {section.title}
      </h2>
      <pre>
        {section.lines.map((line, i) => (
          <Line content={line} key={i} />
        ))}
      </pre>
      {requirements.length ? (
        <>
          <h3>Requirements</h3>
          <Requirements
            key={key}
            requirements={section.requirements}
            showSection
          />
        </>
      ) : null}
    </>
  );
}

function Line({ content }) {
  return (
    <>
      {content.map((reference, i) => {
        // don't highlight text without references
        if (!reference.annotations.length) return reference.text;

        return <Quote reference={reference} key={i} />;
      })}
      <br />
    </>
  );
}

const useStyles = makeStyles((theme) => ({
  paper: {
    margin: theme.spacing(2),
    padding: theme.spacing(2),
  },
  reference: {
    cursor: "pointer",
  },
  error: {
    borderBottom: `2px solid ${theme.palette.error.main}`,
  },
  warning: {
    borderBottom: `2px solid ${theme.palette.warning.main}`,
  },
  ok: {
    borderBottom: `2px solid ${theme.palette.success.main}`,
  },
  neutral: {
    borderBottom: `2px solid ${theme.palette.info.main}`,
  },
  exception: {
    borderBottom: `2px solid ${theme.palette.text.disabled}`,
  },
  selected: {
    backgroundColor: theme.palette.action.focus,
  },
}));

const QuoteTooltip = withStyles((theme) => ({
  tooltip: {
    backgroundColor: theme.palette.background.paper,
    color: theme.palette.text.primary,
    maxWidth: 512,
    fontSize: theme.typography.pxToRem(12),
    border: "1px solid #dadde9",
  },
}))(Tooltip);

function useAnnotationSelection() {
  const { hash } = useLocation();

  return useMemo(() => {
    return new Set(
      (hash || "")
        .replace(/^#/, "")
        .split(",")
        .filter((id) => /^A[\d]+/.test(id))
        .map((id) => parseInt(id.slice(1)))
    );
  }, [hash]);
}

function Quote({ reference }) {
  const { status, text } = reference;
  const classes = useStyles();
  const [open, setOpen] = useState(false);
  const selectedAnnotations = useAnnotationSelection();

  const handleOpen = () => {
    setOpen(true);
  };
  const handleClose = () => {
    setOpen(false);
  };

  let statusClass = "neutral";

  if (status.spec) {
    if (status.citation && status.test) {
      statusClass = "ok";
    } else if (status.citation || status.test) {
      statusClass = "warning";
    } else {
      statusClass = "error";
    }
  }

  if (status.exception) {
    statusClass = "exception";
  }

  let selected =
    selectedAnnotations.size &&
    reference.annotations.find((anno) => selectedAnnotations.has(anno.id));

  return (
    <>
      <QuoteTooltip title={<Annotations reference={reference} />}>
        <span
          className={clsx(classes.reference, classes[statusClass], {
            [classes.selected]: selected,
          })}
          onClick={handleOpen}
        >
          {text}
        </span>
      </QuoteTooltip>
      <Dialog open={open} onClose={handleClose} maxWidth={false}>
        <Paper className={classes.paper}>
          <Annotations reference={reference} expanded />
        </Paper>
      </Dialog>
    </>
  );
}

function Annotations({ reference: { annotations, status }, expanded }) {
  const refs = {
    CITATION: [],
    SPEC: [],
    TEST: [],
    EXCEPTION: [],
    TODO: [],
  };

  annotations.forEach((anno) => {
    if (anno.source) {
      (refs[anno.type || "CITATION"] || []).push(anno);
    }
  });

  const requirement = status.level ? <h3>Level: {status.level}</h3> : null;
  const isOk = !!refs.SPEC.find((ref) => ref.isOk);
  const showMissing = requirement && !isOk;

  const comments = expanded ? refs.SPEC.filter((ref) => ref.comment) : [];

  return (
    <>
      {requirement}
      {comments.map((anno, i) => (
        <Comment annotation={anno} key={anno.id} />
      ))}
      <AnnotationRef
        title="Specifications"
        refs={refs.SPEC.length > 1 ? refs.SPEC : []}
      />
      <AnnotationRef
        title="Citations"
        alt={showMissing && "Missing!"}
        refs={refs.CITATION}
      />
      <AnnotationRef
        title="Tests"
        alt={showMissing && "Missing!"}
        refs={refs.TEST}
      />
      <AnnotationRef
        title="Exceptions"
        refs={refs.EXCEPTION}
        expanded={expanded}
      />
      <AnnotationRef title="TODOs" refs={refs.TODO} />
    </>
  );
}

function AnnotationRef({ title, alt, refs, expanded }) {
  if (!refs.length && !alt) {
    return null;
  }

  const content = refs.length ? (
    refs.map((anno, id) => {
      const text = <ListItemText secondary={anno.source.title} />;
      const content = anno.source.href ? (
        <Link href={anno.source.href}>{text}</Link>
      ) : (
        text
      );
      return (
        <ListItem style={{ padding: "0 8px", display: "block" }} key={id}>
          {content}
          {expanded && anno.comment ? (
            <Typography
              variant="body2"
              style={{ paddingLeft: 16, maxWidth: 500 }}
            >
              {anno.comment}
            </Typography>
          ) : null}
        </ListItem>
      );
    })
  ) : (
    <ListItem style={{ padding: "0 8px", display: "block" }}>
      <Box color="error.main">
        <h4>{alt}</h4>
      </Box>
    </ListItem>
  );

  return (
    <>
      <h4 style={{ lineHeight: 1 }}>{title}</h4>
      <List style={{ padding: 0 }}>{content}</List>
    </>
  );
}

const useCommentStyles = makeStyles((theme) => ({
  list: {
    display: "flex",
    flexWrap: "wrap",
    listStyle: "none",
    justifyContent: "right",
    padding: theme.spacing(0.5),
    margin: 0,
  },
  chip: {
    margin: theme.spacing(0.5),
  },
}));

function Comment({ annotation }) {
  const classes = useCommentStyles();
  return (
    <>
      <p>
        <pre>{annotation.comment}</pre>
      </p>
      <ul className={classes.list}>
        <li style={{ padding: "6px 4px" }}>
          <FileCopyIcon color="primary" />
        </li>
        <li>
          <Cite
            className={classes.chip}
            getData={() => formatReferenceComment({ annotation })}
            label="Citation"
          />
        </li>
        <li>
          <Cite
            className={classes.chip}
            getData={() => formatReferenceComment({ annotation, type: "test" })}
            label="Test"
          />
        </li>
        <li>
          <Cite
            className={classes.chip}
            getData={() => formatTomlComment({ annotation, type: "exception" })}
            label="Exception"
          />
        </li>
        <li>
          <Cite
            className={classes.chip}
            getData={() => formatReferenceComment({ annotation, type: "TODO" })}
            label="TODO"
          />
        </li>
      </ul>
    </>
  );
}

function Cite({ getData, label, ...props }) {
  const [copied, setCopied] = useState(false);

  const onClick = () => {
    copyToClipboard(getData());
    setCopied(true);
    setTimeout(() => setCopied(false), 1000);
  };

  return (
    <Chip
      variant="outlined"
      onClick={onClick}
      label={copied ? `${label} - Copied!` : label}
      {...props}
    />
  );
}

function formatReferenceComment({ annotation, type }) {
  let comment = [];
  comment.push(`//= ${annotation.target}`);
  if (type) comment.push(`//= type=${type}`);
  annotation.comment
    .trim()
    .split("\n")
    .forEach((line) => {
      comment.push(`//# ${line}`);
    });
  return comment.join("\n") + "\n";
}

function formatTomlComment({ annotation, type }) {
  let comment = [`[[${type}]]`];
  comment.push(`target = ${JSON.stringify(annotation.target)}`);
  comment.push("quote = '''");
  comment.push(...annotation.comment.trim().split("\n"));
  comment.push("'''");
  if (type === "exception")
    comment.push('reason = "TODO: Add reason for exception here"');

  return comment.join("\n") + "\n";
}