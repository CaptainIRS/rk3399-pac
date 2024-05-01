#[doc = "Register `SWREG1_INT` reader"]
pub type R = crate::R<Swreg1IntSpec>;
#[doc = "Register `SWREG1_INT` writer"]
pub type W = crate::W<Swreg1IntSpec>;
#[doc = "Field `SW_DEC_E` reader - decoder enable\n\nDecoder enable. Setting this bit high will start the decoding\n\noperation. HW will reset this when the picture is decoded ready or\n\nbus error or time out interrupt is given for all decode format.\n\nHW will reset this when picture is processed stream error for vp9 &amp;\n\nhevc &amp; (h264 when sw_h264_error_mode is 1'b0)"]
pub type SwDecER = crate::BitReader;
#[doc = "Field `SW_DEC_E` writer - decoder enable\n\nDecoder enable. Setting this bit high will start the decoding\n\noperation. HW will reset this when the picture is decoded ready or\n\nbus error or time out interrupt is given for all decode format.\n\nHW will reset this when picture is processed stream error for vp9 &amp;\n\nhevc &amp; (h264 when sw_h264_error_mode is 1'b0)"]
pub type SwDecEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_DEC_CLKGATE_E` reader - decoder dynamic clock gating enable\n\n0 = clock is running for all structures\n\n1 = clock is gated for decoder structures that are not used"]
pub type SwDecClkgateER = crate::BitReader;
#[doc = "Field `SW_DEC_CLKGATE_E` writer - decoder dynamic clock gating enable\n\n0 = clock is running for all structures\n\n1 = clock is gated for decoder structures that are not used"]
pub type SwDecClkgateEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "time out mode\n\ntimeout mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwTimeoutMode {
    #[doc = "0: TIMEOUT_CYCLES is 241'b1"]
    B0 = 0,
    #[doc = "1: TIMEOUT_CYCLES is 181'b1"]
    B1 = 1,
}
impl From<SwTimeoutMode> for bool {
    #[inline(always)]
    fn from(variant: SwTimeoutMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_TIMEOUT_MODE` reader - time out mode\n\ntimeout mode select"]
pub type SwTimeoutModeR = crate::BitReader<SwTimeoutMode>;
impl SwTimeoutModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwTimeoutMode {
        match self.bits {
            false => SwTimeoutMode::B0,
            true => SwTimeoutMode::B1,
        }
    }
    #[doc = "TIMEOUT_CYCLES is 241'b1"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwTimeoutMode::B0
    }
    #[doc = "TIMEOUT_CYCLES is 181'b1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwTimeoutMode::B1
    }
}
#[doc = "Field `SW_TIMEOUT_MODE` writer - time out mode\n\ntimeout mode select"]
pub type SwTimeoutModeW<'a, REG> = crate::BitWriter<'a, REG, SwTimeoutMode>;
impl<'a, REG> SwTimeoutModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMEOUT_CYCLES is 241'b1"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwTimeoutMode::B0)
    }
    #[doc = "TIMEOUT_CYCLES is 181'b1"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwTimeoutMode::B1)
    }
}
#[doc = "Field `SW_DEC_IRQ_DIS` reader - decoder IRQ disable\n\nWhen hight, there are no interrupts concerning decoder from HW.\n\nPolling must be used to see the interrupt status"]
pub type SwDecIrqDisR = crate::BitReader;
#[doc = "Field `SW_DEC_IRQ_DIS` writer - decoder IRQ disable\n\nWhen hight, there are no interrupts concerning decoder from HW.\n\nPolling must be used to see the interrupt status"]
pub type SwDecIrqDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_DEC_TIMEOUT_E` reader - Timeout interrupt enable\n\nIf enabled HW may return timeout interrupt in case HW gets\n\nstucked while decoding picture."]
pub type SwDecTimeoutER = crate::BitReader;
#[doc = "Field `SW_DEC_TIMEOUT_E` writer - Timeout interrupt enable\n\nIf enabled HW may return timeout interrupt in case HW gets\n\nstucked while decoding picture."]
pub type SwDecTimeoutEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_BUF_EMPTY_EN` reader - buffer empty int enable\n\nbuffer empty interrupt enable, now is for no use"]
pub type SwBufEmptyEnR = crate::BitReader;
#[doc = "Field `SW_BUF_EMPTY_EN` writer - buffer empty int enable\n\nbuffer empty interrupt enable, now is for no use"]
pub type SwBufEmptyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_STMERROR_WAITDECFIFO_EMPTY` reader - whether the stream error process wait the decfifo empty\n\nwhen it is 1'b0, the stream error process will no wait the ca2decfifo\n\nempty\n\nwhen it is 1'b1, the stream error process will wait the ca2decfifo\n\nempty\n\nwhen sw_dec_mode is HEVC and VP9, it always take effect; when\n\nsw_dec_mode is H264, it only take effect when\n\nsw_h264_error_mode is 1'b0"]
pub type SwStmerrorWaitdecfifoEmptyR = crate::BitReader;
#[doc = "Field `SW_STMERROR_WAITDECFIFO_EMPTY` writer - whether the stream error process wait the decfifo empty\n\nwhen it is 1'b0, the stream error process will no wait the ca2decfifo\n\nempty\n\nwhen it is 1'b1, the stream error process will wait the ca2decfifo\n\nempty\n\nwhen sw_dec_mode is HEVC and VP9, it always take effect; when\n\nsw_dec_mode is H264, it only take effect when\n\nsw_h264_error_mode is 1'b0"]
pub type SwStmerrorWaitdecfifoEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_DEC_IRQ` reader - decoder IRQ\n\nwhen high, decoder requests an interrrupt.\n\nsw_dec_irq = sw_dec_irq_raw &amp;&amp; (sw_dec_irq_dis == 1'b0)"]
pub type SwDecIrqR = crate::BitReader;
#[doc = "Field `SW_DEC_IRQ_RAW` reader - the raw status of sw_dec_irq\n\nthe raw status of sw_dec_irq,SW should reset this bit after\n\ninterrupt is handled"]
pub type SwDecIrqRawR = crate::BitReader;
#[doc = "Field `SW_DEC_IRQ_RAW` writer - the raw status of sw_dec_irq\n\nthe raw status of sw_dec_irq,SW should reset this bit after\n\ninterrupt is handled"]
pub type SwDecIrqRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_DEC_E_REWRITE_VALID` writer - sw_de_e rewrite valid signal\n\nsw_dec_e rewrite valid signal\n\nmaybe for only when buffer empty, restart the decoder use"]
pub type SwDecERewriteValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_DEC_RDY_STA` reader - decoder ready status\n\nwhen this bit is high, decoder has decoded a picture( the loop filter\n\nmodule send out a frame rdy)"]
pub type SwDecRdyStaR = crate::BitReader;
#[doc = "Field `SW_DEC_RDY_STA` writer - decoder ready status\n\nwhen this bit is high, decoder has decoded a picture( the loop filter\n\nmodule send out a frame rdy)"]
pub type SwDecRdyStaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_DEC_BUS_STA` reader - bus error status\n\nWhen this bit is high, there is error on the axi bus, it will self reset\n\nhardware"]
pub type SwDecBusStaR = crate::BitReader;
#[doc = "Field `SW_DEC_BUS_STA` writer - bus error status\n\nWhen this bit is high, there is error on the axi bus, it will self reset\n\nhardware"]
pub type SwDecBusStaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_DEC_ERROR_STA` reader - status bit of input stream error\n\nhevc &amp; vp9:when high, an error is found in input data stream\n\ndecoding. It will self reset the hardware.\n\nh264: when high, an error is found in input data stream\n\ndecoding.when sw_h264_error_mode is 1'b0, it will self reset the\n\nhardware, otherwise it will not"]
pub type SwDecErrorStaR = crate::BitReader;
#[doc = "Field `SW_DEC_ERROR_STA` writer - status bit of input stream error\n\nhevc &amp; vp9:when high, an error is found in input data stream\n\ndecoding. It will self reset the hardware.\n\nh264: when high, an error is found in input data stream\n\ndecoding.when sw_h264_error_mode is 1'b0, it will self reset the\n\nhardware, otherwise it will not"]
pub type SwDecErrorStaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_DEC_TIMEOUT_STA` reader - decoder timeout interrupt status\n\nWhen high the decoder has been idling for too long. it will self reset\n\nthe hardware\n\nonly when sw_dec_timeout_e is 1'b1, this bit is valid"]
pub type SwDecTimeoutStaR = crate::BitReader;
#[doc = "Field `SW_DEC_TIMEOUT_STA` writer - decoder timeout interrupt status\n\nWhen high the decoder has been idling for too long. it will self reset\n\nthe hardware\n\nonly when sw_dec_timeout_e is 1'b1, this bit is valid"]
pub type SwDecTimeoutStaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_BUF_EMPTY_STA` reader - buffer empty sta\n\nbuffer empty status, only when sw_buf_empty_en is 1'b1 , this bit\n\nis valid, now is for no valid"]
pub type SwBufEmptyStaR = crate::BitReader;
#[doc = "Field `SW_BUF_EMPTY_STA` writer - buffer empty sta\n\nbuffer empty status, only when sw_buf_empty_en is 1'b1 , this bit\n\nis valid, now is for no valid"]
pub type SwBufEmptyStaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_COLMV_REF_ERROR_STA` reader - colmv ref error status\n\ncolmv ref error status\n\nhevc&amp;vp9: when it is 1'b1, it means that inter module read the\n\ninvalid dpb frame\n\nIt will self reset the hardware\n\nh264: when it is 1'b1, it means that inter module read the invalid\n\ndpb frame. when sw_h264_error_mode is 1'b0, it will self reset the\n\nhardware, otherwise it will not"]
pub type SwColmvRefErrorStaR = crate::BitReader;
#[doc = "Field `SW_COLMV_REF_ERROR_STA` writer - colmv ref error status\n\ncolmv ref error status\n\nhevc&amp;vp9: when it is 1'b1, it means that inter module read the\n\ninvalid dpb frame\n\nIt will self reset the hardware\n\nh264: when it is 1'b1, it means that inter module read the invalid\n\ndpb frame. when sw_h264_error_mode is 1'b0, it will self reset the\n\nhardware, otherwise it will not"]
pub type SwColmvRefErrorStaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_CABU_END_STA` reader - cabac decode end status\n\nhevc: cabac decode end status\n\nh264&amp; vp9 : streamd decode status"]
pub type SwCabuEndStaR = crate::BitReader;
#[doc = "Field `SW_CABU_END_STA` writer - cabac decode end status\n\nhevc: cabac decode end status\n\nh264&amp; vp9 : streamd decode status"]
pub type SwCabuEndStaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_H264ORVP9_ERROR_MODE` reader - h264 or vp9 error mode\n\nfor VP9:\n\n1'b0: when there is any stream or colmv error, the hardware will\n\nstop the decode and reset itself\n\n1'b1: when there is any stream or colmv error, the hardware will\n\nstill decode the next slice\n\nit is recommend that when vp9 , it is configed to 1'b0\n\nfor H264:\n\n1'b0: when there is any stream , the hardware will stop the decoder\n\nand reset itself\n\n1'b1: when there is any stream error, the hardware will wait the\n\nend signal of deblocking and then reset itself"]
pub type SwH264orvp9ErrorModeR = crate::BitReader;
#[doc = "Field `SW_H264ORVP9_ERROR_MODE` writer - h264 or vp9 error mode\n\nfor VP9:\n\n1'b0: when there is any stream or colmv error, the hardware will\n\nstop the decode and reset itself\n\n1'b1: when there is any stream or colmv error, the hardware will\n\nstill decode the next slice\n\nit is recommend that when vp9 , it is configed to 1'b0\n\nfor H264:\n\n1'b0: when there is any stream , the hardware will stop the decoder\n\nand reset itself\n\n1'b1: when there is any stream error, the hardware will wait the\n\nend signal of deblocking and then reset itself"]
pub type SwH264orvp9ErrorModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_SOFTRST_EN_P` reader - softreset enable\n\nsoftreset enable signal\n\nwrite 1 to soft reset, write 0 invalid\n\npuls register"]
pub type SwSoftrstEnPR = crate::BitReader;
#[doc = "Field `SW_SOFTRST_EN_P` writer - softreset enable\n\nsoftreset enable signal\n\nwrite 1 to soft reset, write 0 invalid\n\npuls register"]
pub type SwSoftrstEnPW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_SOFTRESET_RDY` reader - when it is 1'b1, it says that softreset has been done"]
pub type SwSoftresetRdyR = crate::BitReader;
#[doc = "Field `SW_SOFTRESET_RDY` writer - when it is 1'b1, it says that softreset has been done"]
pub type SwSoftresetRdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - decoder enable\n\nDecoder enable. Setting this bit high will start the decoding\n\noperation. HW will reset this when the picture is decoded ready or\n\nbus error or time out interrupt is given for all decode format.\n\nHW will reset this when picture is processed stream error for vp9 &amp;\n\nhevc &amp; (h264 when sw_h264_error_mode is 1'b0)"]
    #[inline(always)]
    pub fn sw_dec_e(&self) -> SwDecER {
        SwDecER::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - decoder dynamic clock gating enable\n\n0 = clock is running for all structures\n\n1 = clock is gated for decoder structures that are not used"]
    #[inline(always)]
    pub fn sw_dec_clkgate_e(&self) -> SwDecClkgateER {
        SwDecClkgateER::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - time out mode\n\ntimeout mode select"]
    #[inline(always)]
    pub fn sw_timeout_mode(&self) -> SwTimeoutModeR {
        SwTimeoutModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - decoder IRQ disable\n\nWhen hight, there are no interrupts concerning decoder from HW.\n\nPolling must be used to see the interrupt status"]
    #[inline(always)]
    pub fn sw_dec_irq_dis(&self) -> SwDecIrqDisR {
        SwDecIrqDisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timeout interrupt enable\n\nIf enabled HW may return timeout interrupt in case HW gets\n\nstucked while decoding picture."]
    #[inline(always)]
    pub fn sw_dec_timeout_e(&self) -> SwDecTimeoutER {
        SwDecTimeoutER::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - buffer empty int enable\n\nbuffer empty interrupt enable, now is for no use"]
    #[inline(always)]
    pub fn sw_buf_empty_en(&self) -> SwBufEmptyEnR {
        SwBufEmptyEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - whether the stream error process wait the decfifo empty\n\nwhen it is 1'b0, the stream error process will no wait the ca2decfifo\n\nempty\n\nwhen it is 1'b1, the stream error process will wait the ca2decfifo\n\nempty\n\nwhen sw_dec_mode is HEVC and VP9, it always take effect; when\n\nsw_dec_mode is H264, it only take effect when\n\nsw_h264_error_mode is 1'b0"]
    #[inline(always)]
    pub fn sw_stmerror_waitdecfifo_empty(&self) -> SwStmerrorWaitdecfifoEmptyR {
        SwStmerrorWaitdecfifoEmptyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - decoder IRQ\n\nwhen high, decoder requests an interrrupt.\n\nsw_dec_irq = sw_dec_irq_raw &amp;&amp; (sw_dec_irq_dis == 1'b0)"]
    #[inline(always)]
    pub fn sw_dec_irq(&self) -> SwDecIrqR {
        SwDecIrqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - the raw status of sw_dec_irq\n\nthe raw status of sw_dec_irq,SW should reset this bit after\n\ninterrupt is handled"]
    #[inline(always)]
    pub fn sw_dec_irq_raw(&self) -> SwDecIrqRawR {
        SwDecIrqRawR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - decoder ready status\n\nwhen this bit is high, decoder has decoded a picture( the loop filter\n\nmodule send out a frame rdy)"]
    #[inline(always)]
    pub fn sw_dec_rdy_sta(&self) -> SwDecRdyStaR {
        SwDecRdyStaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - bus error status\n\nWhen this bit is high, there is error on the axi bus, it will self reset\n\nhardware"]
    #[inline(always)]
    pub fn sw_dec_bus_sta(&self) -> SwDecBusStaR {
        SwDecBusStaR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - status bit of input stream error\n\nhevc &amp; vp9:when high, an error is found in input data stream\n\ndecoding. It will self reset the hardware.\n\nh264: when high, an error is found in input data stream\n\ndecoding.when sw_h264_error_mode is 1'b0, it will self reset the\n\nhardware, otherwise it will not"]
    #[inline(always)]
    pub fn sw_dec_error_sta(&self) -> SwDecErrorStaR {
        SwDecErrorStaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - decoder timeout interrupt status\n\nWhen high the decoder has been idling for too long. it will self reset\n\nthe hardware\n\nonly when sw_dec_timeout_e is 1'b1, this bit is valid"]
    #[inline(always)]
    pub fn sw_dec_timeout_sta(&self) -> SwDecTimeoutStaR {
        SwDecTimeoutStaR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - buffer empty sta\n\nbuffer empty status, only when sw_buf_empty_en is 1'b1 , this bit\n\nis valid, now is for no valid"]
    #[inline(always)]
    pub fn sw_buf_empty_sta(&self) -> SwBufEmptyStaR {
        SwBufEmptyStaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - colmv ref error status\n\ncolmv ref error status\n\nhevc&amp;vp9: when it is 1'b1, it means that inter module read the\n\ninvalid dpb frame\n\nIt will self reset the hardware\n\nh264: when it is 1'b1, it means that inter module read the invalid\n\ndpb frame. when sw_h264_error_mode is 1'b0, it will self reset the\n\nhardware, otherwise it will not"]
    #[inline(always)]
    pub fn sw_colmv_ref_error_sta(&self) -> SwColmvRefErrorStaR {
        SwColmvRefErrorStaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - cabac decode end status\n\nhevc: cabac decode end status\n\nh264&amp; vp9 : streamd decode status"]
    #[inline(always)]
    pub fn sw_cabu_end_sta(&self) -> SwCabuEndStaR {
        SwCabuEndStaR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - h264 or vp9 error mode\n\nfor VP9:\n\n1'b0: when there is any stream or colmv error, the hardware will\n\nstop the decode and reset itself\n\n1'b1: when there is any stream or colmv error, the hardware will\n\nstill decode the next slice\n\nit is recommend that when vp9 , it is configed to 1'b0\n\nfor H264:\n\n1'b0: when there is any stream , the hardware will stop the decoder\n\nand reset itself\n\n1'b1: when there is any stream error, the hardware will wait the\n\nend signal of deblocking and then reset itself"]
    #[inline(always)]
    pub fn sw_h264orvp9_error_mode(&self) -> SwH264orvp9ErrorModeR {
        SwH264orvp9ErrorModeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - softreset enable\n\nsoftreset enable signal\n\nwrite 1 to soft reset, write 0 invalid\n\npuls register"]
    #[inline(always)]
    pub fn sw_softrst_en_p(&self) -> SwSoftrstEnPR {
        SwSoftrstEnPR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - when it is 1'b1, it says that softreset has been done"]
    #[inline(always)]
    pub fn sw_softreset_rdy(&self) -> SwSoftresetRdyR {
        SwSoftresetRdyR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - decoder enable\n\nDecoder enable. Setting this bit high will start the decoding\n\noperation. HW will reset this when the picture is decoded ready or\n\nbus error or time out interrupt is given for all decode format.\n\nHW will reset this when picture is processed stream error for vp9 &amp;\n\nhevc &amp; (h264 when sw_h264_error_mode is 1'b0)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_e(&mut self) -> SwDecEW<Swreg1IntSpec> {
        SwDecEW::new(self, 0)
    }
    #[doc = "Bit 1 - decoder dynamic clock gating enable\n\n0 = clock is running for all structures\n\n1 = clock is gated for decoder structures that are not used"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_clkgate_e(&mut self) -> SwDecClkgateEW<Swreg1IntSpec> {
        SwDecClkgateEW::new(self, 1)
    }
    #[doc = "Bit 3 - time out mode\n\ntimeout mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sw_timeout_mode(&mut self) -> SwTimeoutModeW<Swreg1IntSpec> {
        SwTimeoutModeW::new(self, 3)
    }
    #[doc = "Bit 4 - decoder IRQ disable\n\nWhen hight, there are no interrupts concerning decoder from HW.\n\nPolling must be used to see the interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_irq_dis(&mut self) -> SwDecIrqDisW<Swreg1IntSpec> {
        SwDecIrqDisW::new(self, 4)
    }
    #[doc = "Bit 5 - Timeout interrupt enable\n\nIf enabled HW may return timeout interrupt in case HW gets\n\nstucked while decoding picture."]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_timeout_e(&mut self) -> SwDecTimeoutEW<Swreg1IntSpec> {
        SwDecTimeoutEW::new(self, 5)
    }
    #[doc = "Bit 6 - buffer empty int enable\n\nbuffer empty interrupt enable, now is for no use"]
    #[inline(always)]
    #[must_use]
    pub fn sw_buf_empty_en(&mut self) -> SwBufEmptyEnW<Swreg1IntSpec> {
        SwBufEmptyEnW::new(self, 6)
    }
    #[doc = "Bit 7 - whether the stream error process wait the decfifo empty\n\nwhen it is 1'b0, the stream error process will no wait the ca2decfifo\n\nempty\n\nwhen it is 1'b1, the stream error process will wait the ca2decfifo\n\nempty\n\nwhen sw_dec_mode is HEVC and VP9, it always take effect; when\n\nsw_dec_mode is H264, it only take effect when\n\nsw_h264_error_mode is 1'b0"]
    #[inline(always)]
    #[must_use]
    pub fn sw_stmerror_waitdecfifo_empty(&mut self) -> SwStmerrorWaitdecfifoEmptyW<Swreg1IntSpec> {
        SwStmerrorWaitdecfifoEmptyW::new(self, 7)
    }
    #[doc = "Bit 9 - the raw status of sw_dec_irq\n\nthe raw status of sw_dec_irq,SW should reset this bit after\n\ninterrupt is handled"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_irq_raw(&mut self) -> SwDecIrqRawW<Swreg1IntSpec> {
        SwDecIrqRawW::new(self, 9)
    }
    #[doc = "Bit 10 - sw_de_e rewrite valid signal\n\nsw_dec_e rewrite valid signal\n\nmaybe for only when buffer empty, restart the decoder use"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_e_rewrite_valid(&mut self) -> SwDecERewriteValidW<Swreg1IntSpec> {
        SwDecERewriteValidW::new(self, 10)
    }
    #[doc = "Bit 12 - decoder ready status\n\nwhen this bit is high, decoder has decoded a picture( the loop filter\n\nmodule send out a frame rdy)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_rdy_sta(&mut self) -> SwDecRdyStaW<Swreg1IntSpec> {
        SwDecRdyStaW::new(self, 12)
    }
    #[doc = "Bit 13 - bus error status\n\nWhen this bit is high, there is error on the axi bus, it will self reset\n\nhardware"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_bus_sta(&mut self) -> SwDecBusStaW<Swreg1IntSpec> {
        SwDecBusStaW::new(self, 13)
    }
    #[doc = "Bit 14 - status bit of input stream error\n\nhevc &amp; vp9:when high, an error is found in input data stream\n\ndecoding. It will self reset the hardware.\n\nh264: when high, an error is found in input data stream\n\ndecoding.when sw_h264_error_mode is 1'b0, it will self reset the\n\nhardware, otherwise it will not"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_error_sta(&mut self) -> SwDecErrorStaW<Swreg1IntSpec> {
        SwDecErrorStaW::new(self, 14)
    }
    #[doc = "Bit 15 - decoder timeout interrupt status\n\nWhen high the decoder has been idling for too long. it will self reset\n\nthe hardware\n\nonly when sw_dec_timeout_e is 1'b1, this bit is valid"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_timeout_sta(&mut self) -> SwDecTimeoutStaW<Swreg1IntSpec> {
        SwDecTimeoutStaW::new(self, 15)
    }
    #[doc = "Bit 16 - buffer empty sta\n\nbuffer empty status, only when sw_buf_empty_en is 1'b1 , this bit\n\nis valid, now is for no valid"]
    #[inline(always)]
    #[must_use]
    pub fn sw_buf_empty_sta(&mut self) -> SwBufEmptyStaW<Swreg1IntSpec> {
        SwBufEmptyStaW::new(self, 16)
    }
    #[doc = "Bit 17 - colmv ref error status\n\ncolmv ref error status\n\nhevc&amp;vp9: when it is 1'b1, it means that inter module read the\n\ninvalid dpb frame\n\nIt will self reset the hardware\n\nh264: when it is 1'b1, it means that inter module read the invalid\n\ndpb frame. when sw_h264_error_mode is 1'b0, it will self reset the\n\nhardware, otherwise it will not"]
    #[inline(always)]
    #[must_use]
    pub fn sw_colmv_ref_error_sta(&mut self) -> SwColmvRefErrorStaW<Swreg1IntSpec> {
        SwColmvRefErrorStaW::new(self, 17)
    }
    #[doc = "Bit 18 - cabac decode end status\n\nhevc: cabac decode end status\n\nh264&amp; vp9 : streamd decode status"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cabu_end_sta(&mut self) -> SwCabuEndStaW<Swreg1IntSpec> {
        SwCabuEndStaW::new(self, 18)
    }
    #[doc = "Bit 19 - h264 or vp9 error mode\n\nfor VP9:\n\n1'b0: when there is any stream or colmv error, the hardware will\n\nstop the decode and reset itself\n\n1'b1: when there is any stream or colmv error, the hardware will\n\nstill decode the next slice\n\nit is recommend that when vp9 , it is configed to 1'b0\n\nfor H264:\n\n1'b0: when there is any stream , the hardware will stop the decoder\n\nand reset itself\n\n1'b1: when there is any stream error, the hardware will wait the\n\nend signal of deblocking and then reset itself"]
    #[inline(always)]
    #[must_use]
    pub fn sw_h264orvp9_error_mode(&mut self) -> SwH264orvp9ErrorModeW<Swreg1IntSpec> {
        SwH264orvp9ErrorModeW::new(self, 19)
    }
    #[doc = "Bit 20 - softreset enable\n\nsoftreset enable signal\n\nwrite 1 to soft reset, write 0 invalid\n\npuls register"]
    #[inline(always)]
    #[must_use]
    pub fn sw_softrst_en_p(&mut self) -> SwSoftrstEnPW<Swreg1IntSpec> {
        SwSoftrstEnPW::new(self, 20)
    }
    #[doc = "Bit 22 - when it is 1'b1, it says that softreset has been done"]
    #[inline(always)]
    #[must_use]
    pub fn sw_softreset_rdy(&mut self) -> SwSoftresetRdyW<Swreg1IntSpec> {
        SwSoftresetRdyW::new(self, 22)
    }
}
#[doc = "interrupt and decoder enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg1_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg1_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg1IntSpec;
impl crate::RegisterSpec for Swreg1IntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg1_int::R`](R) reader structure"]
impl crate::Readable for Swreg1IntSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg1_int::W`](W) writer structure"]
impl crate::Writable for Swreg1IntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG1_INT to value 0x22"]
impl crate::Resettable for Swreg1IntSpec {
    const RESET_VALUE: u32 = 0x22;
}
