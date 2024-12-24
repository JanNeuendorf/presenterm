use crate::{
    markdown::elements::Line,
    presentation::{AsRenderOperations, BlockLine, RenderOperation},
    render::{
        layout::{Layout, Positioning},
        properties::WindowSize,
    },
    theme::{Alignment, Margin},
};
use std::rc::Rc;

#[derive(Clone, Debug, Default)]
pub(crate) enum SeparatorWidth {
    Fixed(u16),

    #[default]
    FitToWindow,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct RenderSeparator {
    heading: Line,
    width: SeparatorWidth,
}

impl RenderSeparator {
    pub(crate) fn new<S: Into<Line>>(heading: S, width: SeparatorWidth) -> Self {
        Self { heading: heading.into(), width }
    }
}

impl From<RenderSeparator> for RenderOperation {
    fn from(separator: RenderSeparator) -> Self {
        Self::RenderDynamic(Rc::new(separator))
    }
}

impl AsRenderOperations for RenderSeparator {
    fn as_render_operations(&self, dimensions: &WindowSize) -> Vec<RenderOperation> {
        let character = "—";
        let width = match self.width {
            SeparatorWidth::Fixed(width) => {
                let Positioning { max_line_length, .. } =
                    Layout::new(Alignment::Center { minimum_margin: Margin::Fixed(0), minimum_size: 0 })
                        .compute(dimensions, width);
                max_line_length.min(width) as usize
            }
            SeparatorWidth::FitToWindow => dimensions.columns as usize,
        };
        let separator = match self.heading.width() == 0 {
            true => Line::from(character.repeat(width)),
            false => {
                let width = width.saturating_sub(self.heading.width());
                let (dashes_len, remainder) = (width / 2, width % 2);
                let mut dashes = character.repeat(dashes_len);
                let mut line = Line::from(dashes.clone());
                line.0.extend(self.heading.0.iter().cloned());

                if remainder > 0 {
                    dashes.push_str(character);
                }
                line.0.push(dashes.into());
                line
            }
        };
        vec![RenderOperation::RenderBlockLine(BlockLine {
            prefix: "".into(),
            right_padding_length: 0,
            repeat_prefix_on_wrap: false,
            text: separator.into(),
            block_length: width as u16,
            block_color: None,
            alignment: Alignment::Center { minimum_size: 1, minimum_margin: Margin::Fixed(0) },
        })]
    }
}
