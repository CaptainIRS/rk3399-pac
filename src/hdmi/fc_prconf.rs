#[doc = "Register `FC_PRCONF` reader"]
pub type R = crate::R<FcPrconfSpec>;
#[doc = "Register `FC_PRCONF` writer"]
pub type W = crate::W<FcPrconfSpec>;
#[doc = "Configures the video pixel repetition ratio to be sent on the AVI InfoFrame. This value must be valid according to the HDMI specification. The output_pr_factor = incoming_pr_factor * (desired_pr_factor + 1) – 1. output_pr_factor\\[3:0\\]
0000b: No action. Not used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OutputPrFactor {
    #[doc = "1: Pixel sent four times 0100b: Pixel sent five times 0101b: Pixel sent six times 0110b: Pixel sent seven times 0111b: Pixel sent eight times 1000b: Pixel sent nine times 1001b: Pixel sent 10 times Other: Reserved. Not used Note: When working in YCC422 video, the actual repetition of the stream is Incoming_pr_factor * (desired_pr_factor + 1). This calculation is done internally in the H13TCTRL and no hardware overflow protection is available. Care must be taken to avoid this result passes the maximum number of 10 pixels repeated because no HDMI support is available for this in the specification and the H13TPHY does not support this higher repetition values."]
    B0001 = 1,
    #[doc = "2: Pixel sent four times 0100b: Pixel sent five times 0101b: Pixel sent six times 0110b: Pixel sent seven times 0111b: Pixel sent eight times 1000b: Pixel sent nine times 1001b: Pixel sent 10 times Other: Reserved. Not used Note: When working in YCC422 video, the actual repetition of the stream is Incoming_pr_factor * (desired_pr_factor + 1). This calculation is done internally in the H13TCTRL and no hardware overflow protection is available. Care must be taken to avoid this result passes the maximum number of 10 pixels repeated because no HDMI support is available for this in the specification and the H13TPHY does not support this higher repetition values."]
    B0010 = 2,
    #[doc = "3: Pixel sent four times 0100b: Pixel sent five times 0101b: Pixel sent six times 0110b: Pixel sent seven times 0111b: Pixel sent eight times 1000b: Pixel sent nine times 1001b: Pixel sent 10 times Other: Reserved. Not used Note: When working in YCC422 video, the actual repetition of the stream is Incoming_pr_factor * (desired_pr_factor + 1). This calculation is done internally in the H13TCTRL and no hardware overflow protection is available. Care must be taken to avoid this result passes the maximum number of 10 pixels repeated because no HDMI support is available for this in the specification and the H13TPHY does not support this higher repetition values."]
    B0011 = 3,
}
impl From<OutputPrFactor> for u8 {
    #[inline(always)]
    fn from(variant: OutputPrFactor) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OutputPrFactor {
    type Ux = u8;
}
#[doc = "Field `OUTPUT_PR_FACTOR` reader - Configures the video pixel repetition ratio to be sent on the AVI InfoFrame. This value must be valid according to the HDMI specification. The output_pr_factor = incoming_pr_factor * (desired_pr_factor + 1) – 1. output_pr_factor\\[3:0\\]
0000b: No action. Not used."]
pub type OutputPrFactorR = crate::FieldReader<OutputPrFactor>;
impl OutputPrFactorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OutputPrFactor> {
        match self.bits {
            1 => Some(OutputPrFactor::B0001),
            2 => Some(OutputPrFactor::B0010),
            3 => Some(OutputPrFactor::B0011),
            _ => None,
        }
    }
    #[doc = "Pixel sent four times 0100b: Pixel sent five times 0101b: Pixel sent six times 0110b: Pixel sent seven times 0111b: Pixel sent eight times 1000b: Pixel sent nine times 1001b: Pixel sent 10 times Other: Reserved. Not used Note: When working in YCC422 video, the actual repetition of the stream is Incoming_pr_factor * (desired_pr_factor + 1). This calculation is done internally in the H13TCTRL and no hardware overflow protection is available. Care must be taken to avoid this result passes the maximum number of 10 pixels repeated because no HDMI support is available for this in the specification and the H13TPHY does not support this higher repetition values."]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == OutputPrFactor::B0001
    }
    #[doc = "Pixel sent four times 0100b: Pixel sent five times 0101b: Pixel sent six times 0110b: Pixel sent seven times 0111b: Pixel sent eight times 1000b: Pixel sent nine times 1001b: Pixel sent 10 times Other: Reserved. Not used Note: When working in YCC422 video, the actual repetition of the stream is Incoming_pr_factor * (desired_pr_factor + 1). This calculation is done internally in the H13TCTRL and no hardware overflow protection is available. Care must be taken to avoid this result passes the maximum number of 10 pixels repeated because no HDMI support is available for this in the specification and the H13TPHY does not support this higher repetition values."]
    #[inline(always)]
    pub fn is_b0010(&self) -> bool {
        *self == OutputPrFactor::B0010
    }
    #[doc = "Pixel sent four times 0100b: Pixel sent five times 0101b: Pixel sent six times 0110b: Pixel sent seven times 0111b: Pixel sent eight times 1000b: Pixel sent nine times 1001b: Pixel sent 10 times Other: Reserved. Not used Note: When working in YCC422 video, the actual repetition of the stream is Incoming_pr_factor * (desired_pr_factor + 1). This calculation is done internally in the H13TCTRL and no hardware overflow protection is available. Care must be taken to avoid this result passes the maximum number of 10 pixels repeated because no HDMI support is available for this in the specification and the H13TPHY does not support this higher repetition values."]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == OutputPrFactor::B0011
    }
}
#[doc = "Field `OUTPUT_PR_FACTOR` writer - Configures the video pixel repetition ratio to be sent on the AVI InfoFrame. This value must be valid according to the HDMI specification. The output_pr_factor = incoming_pr_factor * (desired_pr_factor + 1) – 1. output_pr_factor\\[3:0\\]
0000b: No action. Not used."]
pub type OutputPrFactorW<'a, REG> = crate::FieldWriter<'a, REG, 4, OutputPrFactor>;
impl<'a, REG> OutputPrFactorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pixel sent four times 0100b: Pixel sent five times 0101b: Pixel sent six times 0110b: Pixel sent seven times 0111b: Pixel sent eight times 1000b: Pixel sent nine times 1001b: Pixel sent 10 times Other: Reserved. Not used Note: When working in YCC422 video, the actual repetition of the stream is Incoming_pr_factor * (desired_pr_factor + 1). This calculation is done internally in the H13TCTRL and no hardware overflow protection is available. Care must be taken to avoid this result passes the maximum number of 10 pixels repeated because no HDMI support is available for this in the specification and the H13TPHY does not support this higher repetition values."]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(OutputPrFactor::B0001)
    }
    #[doc = "Pixel sent four times 0100b: Pixel sent five times 0101b: Pixel sent six times 0110b: Pixel sent seven times 0111b: Pixel sent eight times 1000b: Pixel sent nine times 1001b: Pixel sent 10 times Other: Reserved. Not used Note: When working in YCC422 video, the actual repetition of the stream is Incoming_pr_factor * (desired_pr_factor + 1). This calculation is done internally in the H13TCTRL and no hardware overflow protection is available. Care must be taken to avoid this result passes the maximum number of 10 pixels repeated because no HDMI support is available for this in the specification and the H13TPHY does not support this higher repetition values."]
    #[inline(always)]
    pub fn b0010(self) -> &'a mut crate::W<REG> {
        self.variant(OutputPrFactor::B0010)
    }
    #[doc = "Pixel sent four times 0100b: Pixel sent five times 0101b: Pixel sent six times 0110b: Pixel sent seven times 0111b: Pixel sent eight times 1000b: Pixel sent nine times 1001b: Pixel sent 10 times Other: Reserved. Not used Note: When working in YCC422 video, the actual repetition of the stream is Incoming_pr_factor * (desired_pr_factor + 1). This calculation is done internally in the H13TCTRL and no hardware overflow protection is available. Care must be taken to avoid this result passes the maximum number of 10 pixels repeated because no HDMI support is available for this in the specification and the H13TPHY does not support this higher repetition values."]
    #[inline(always)]
    pub fn b0011(self) -> &'a mut crate::W<REG> {
        self.variant(OutputPrFactor::B0011)
    }
}
#[doc = "Configures the input video pixel repetition. For CEA Bits Name Attr Description modes, this value must be extracted from the CEA specification for the video mode being input. incoming_pr_factor\\[3:0\\]
0000b: No action. Not used.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IncomingPrFactor {
    #[doc = "1: Pixel sent four times 0101b: Pixel sent five times 0110b: Pixel sent six times 0111b: Pixel sent seven times 1000b: Pixel sent eight times 1001b: Pixel sent nine times 1010b: Pixel sent 10 times Other: Reserved. Not used"]
    B0001 = 1,
    #[doc = "2: Pixel sent four times 0101b: Pixel sent five times 0110b: Pixel sent six times 0111b: Pixel sent seven times 1000b: Pixel sent eight times 1001b: Pixel sent nine times 1010b: Pixel sent 10 times Other: Reserved. Not used"]
    B0010 = 2,
    #[doc = "3: Pixel sent four times 0101b: Pixel sent five times 0110b: Pixel sent six times 0111b: Pixel sent seven times 1000b: Pixel sent eight times 1001b: Pixel sent nine times 1010b: Pixel sent 10 times Other: Reserved. Not used"]
    B0011 = 3,
    #[doc = "4: Pixel sent four times 0101b: Pixel sent five times 0110b: Pixel sent six times 0111b: Pixel sent seven times 1000b: Pixel sent eight times 1001b: Pixel sent nine times 1010b: Pixel sent 10 times Other: Reserved. Not used"]
    B0100 = 4,
}
impl From<IncomingPrFactor> for u8 {
    #[inline(always)]
    fn from(variant: IncomingPrFactor) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IncomingPrFactor {
    type Ux = u8;
}
#[doc = "Field `INCOMING_PR_FACTOR` reader - Configures the input video pixel repetition. For CEA Bits Name Attr Description modes, this value must be extracted from the CEA specification for the video mode being input. incoming_pr_factor\\[3:0\\]
0000b: No action. Not used."]
pub type IncomingPrFactorR = crate::FieldReader<IncomingPrFactor>;
impl IncomingPrFactorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IncomingPrFactor> {
        match self.bits {
            1 => Some(IncomingPrFactor::B0001),
            2 => Some(IncomingPrFactor::B0010),
            3 => Some(IncomingPrFactor::B0011),
            4 => Some(IncomingPrFactor::B0100),
            _ => None,
        }
    }
    #[doc = "Pixel sent four times 0101b: Pixel sent five times 0110b: Pixel sent six times 0111b: Pixel sent seven times 1000b: Pixel sent eight times 1001b: Pixel sent nine times 1010b: Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == IncomingPrFactor::B0001
    }
    #[doc = "Pixel sent four times 0101b: Pixel sent five times 0110b: Pixel sent six times 0111b: Pixel sent seven times 1000b: Pixel sent eight times 1001b: Pixel sent nine times 1010b: Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn is_b0010(&self) -> bool {
        *self == IncomingPrFactor::B0010
    }
    #[doc = "Pixel sent four times 0101b: Pixel sent five times 0110b: Pixel sent six times 0111b: Pixel sent seven times 1000b: Pixel sent eight times 1001b: Pixel sent nine times 1010b: Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == IncomingPrFactor::B0011
    }
    #[doc = "Pixel sent four times 0101b: Pixel sent five times 0110b: Pixel sent six times 0111b: Pixel sent seven times 1000b: Pixel sent eight times 1001b: Pixel sent nine times 1010b: Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == IncomingPrFactor::B0100
    }
}
#[doc = "Field `INCOMING_PR_FACTOR` writer - Configures the input video pixel repetition. For CEA Bits Name Attr Description modes, this value must be extracted from the CEA specification for the video mode being input. incoming_pr_factor\\[3:0\\]
0000b: No action. Not used."]
pub type IncomingPrFactorW<'a, REG> = crate::FieldWriter<'a, REG, 4, IncomingPrFactor>;
impl<'a, REG> IncomingPrFactorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pixel sent four times 0101b: Pixel sent five times 0110b: Pixel sent six times 0111b: Pixel sent seven times 1000b: Pixel sent eight times 1001b: Pixel sent nine times 1010b: Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(IncomingPrFactor::B0001)
    }
    #[doc = "Pixel sent four times 0101b: Pixel sent five times 0110b: Pixel sent six times 0111b: Pixel sent seven times 1000b: Pixel sent eight times 1001b: Pixel sent nine times 1010b: Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn b0010(self) -> &'a mut crate::W<REG> {
        self.variant(IncomingPrFactor::B0010)
    }
    #[doc = "Pixel sent four times 0101b: Pixel sent five times 0110b: Pixel sent six times 0111b: Pixel sent seven times 1000b: Pixel sent eight times 1001b: Pixel sent nine times 1010b: Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn b0011(self) -> &'a mut crate::W<REG> {
        self.variant(IncomingPrFactor::B0011)
    }
    #[doc = "Pixel sent four times 0101b: Pixel sent five times 0110b: Pixel sent six times 0111b: Pixel sent seven times 1000b: Pixel sent eight times 1001b: Pixel sent nine times 1010b: Pixel sent 10 times Other: Reserved. Not used"]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(IncomingPrFactor::B0100)
    }
}
impl R {
    #[doc = "Bits 0:3 - Configures the video pixel repetition ratio to be sent on the AVI InfoFrame. This value must be valid according to the HDMI specification. The output_pr_factor = incoming_pr_factor * (desired_pr_factor + 1) – 1. output_pr_factor\\[3:0\\]
0000b: No action. Not used."]
    #[inline(always)]
    pub fn output_pr_factor(&self) -> OutputPrFactorR {
        OutputPrFactorR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Configures the input video pixel repetition. For CEA Bits Name Attr Description modes, this value must be extracted from the CEA specification for the video mode being input. incoming_pr_factor\\[3:0\\]
0000b: No action. Not used."]
    #[inline(always)]
    pub fn incoming_pr_factor(&self) -> IncomingPrFactorR {
        IncomingPrFactorR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures the video pixel repetition ratio to be sent on the AVI InfoFrame. This value must be valid according to the HDMI specification. The output_pr_factor = incoming_pr_factor * (desired_pr_factor + 1) – 1. output_pr_factor\\[3:0\\]
0000b: No action. Not used."]
    #[inline(always)]
    #[must_use]
    pub fn output_pr_factor(&mut self) -> OutputPrFactorW<FcPrconfSpec> {
        OutputPrFactorW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Configures the input video pixel repetition. For CEA Bits Name Attr Description modes, this value must be extracted from the CEA specification for the video mode being input. incoming_pr_factor\\[3:0\\]
0000b: No action. Not used."]
    #[inline(always)]
    #[must_use]
    pub fn incoming_pr_factor(&mut self) -> IncomingPrFactorW<FcPrconfSpec> {
        IncomingPrFactorW::new(self, 4)
    }
}
#[doc = "Configures the video pixel repetition ratio to be sent on the AVI InfoFrame. This value must be valid according to the HDMI specification. The output_pr_factor = incoming_pr_factor * (desired_pr_factor + 1) – 1. output_pr_factor\\[3:0\\]
0000b: No action. Not used. 0001b: Pixel sent two times (pixel repeated once) 0010b: Pixel sent three times 0011b: Pixel sent four times 0100b: Pixel sent five times 0101b: Pixel sent six times 0110b: Pixel sent seven times 0111b: Pixel sent eight times 1000b: Pixel sent nine times 1001b: Pixel sent 10 times Other: Reserved. Not used Note: When working in YCC422 video, the actual repetition of the stream is Incoming_pr_factor * (desired_pr_factor + 1). This calculation is done internally in the H13TCTRL and no hardware overflow protection is available. Care must be taken to avoid this result passes the maximum number of 10 pixels repeated because no HDMI support is available for this in the specification and the H13TPHY does not support this higher repetition values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_prconf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_prconf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcPrconfSpec;
impl crate::RegisterSpec for FcPrconfSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_prconf::R`](R) reader structure"]
impl crate::Readable for FcPrconfSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_prconf::W`](W) writer structure"]
impl crate::Writable for FcPrconfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_PRCONF to value 0x10"]
impl crate::Resettable for FcPrconfSpec {
    const RESET_VALUE: u8 = 0x10;
}
