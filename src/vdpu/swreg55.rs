#[doc = "Register `SWREG55` reader"]
pub type R = crate::R<Swreg55Spec>;
#[doc = "Register `SWREG55` writer"]
pub type W = crate::W<Swreg55Spec>;
#[doc = "Field `SW_DEC_IRQ` reader - the decoder finish interrupt request\n\nafter sw query this interrupt,shoud write 0 to reset."]
pub type SwDecIrqR = crate::BitReader;
#[doc = "Field `SW_DEC_IRQ` writer - the decoder finish interrupt request\n\nafter sw query this interrupt,shoud write 0 to reset."]
pub type SwDecIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "the decoder finish interrupt request diable flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDecIrqDis {
    #[doc = "1: use polling to see the interrupt"]
    B1 = 1,
    #[doc = "0: use sw_pp_irq"]
    B0 = 0,
}
impl From<SwDecIrqDis> for bool {
    #[inline(always)]
    fn from(variant: SwDecIrqDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DEC_IRQ_DIS` reader - the decoder finish interrupt request diable flag"]
pub type SwDecIrqDisR = crate::BitReader<SwDecIrqDis>;
impl SwDecIrqDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDecIrqDis {
        match self.bits {
            true => SwDecIrqDis::B1,
            false => SwDecIrqDis::B0,
        }
    }
    #[doc = "use polling to see the interrupt"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDecIrqDis::B1
    }
    #[doc = "use sw_pp_irq"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDecIrqDis::B0
    }
}
#[doc = "Field `SW_DEC_IRQ_DIS` writer - the decoder finish interrupt request diable flag"]
pub type SwDecIrqDisW<'a, REG> = crate::BitWriter<'a, REG, SwDecIrqDis>;
impl<'a, REG> SwDecIrqDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "use polling to see the interrupt"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecIrqDis::B1)
    }
    #[doc = "use sw_pp_irq"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecIrqDis::B0)
    }
}
#[doc = "Field `SW_DEC_RDY_STS` reader - the Interrupt status bit for tell sw processed a picture"]
pub type SwDecRdyStsR = crate::BitReader;
#[doc = "Field `SW_DEC_RDY_STS` writer - the Interrupt status bit for tell sw processed a picture"]
pub type SwDecRdyStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_PP_BUS_STS` reader - the Interrupt status bit for tell sw bus have some error"]
pub type SwPpBusStsR = crate::BitReader;
#[doc = "Field `SW_PP_BUS_STS` writer - the Interrupt status bit for tell sw bus have some error"]
pub type SwPpBusStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_BUF_EMT_STS` reader - the Interrupt status bit for tell input buffer empty"]
pub type SwBufEmtStsR = crate::BitReader;
#[doc = "Field `SW_BUF_EMT_STS` writer - the Interrupt status bit for tell input buffer empty"]
pub type SwBufEmtStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_ASO_DET_STS` reader - the Interrupt status bit for tell us ASO detectet\n\nASO:Arbitrary Slice Ordering"]
pub type SwAsoDetStsR = crate::BitReader;
#[doc = "Field `SW_ASO_DET_STS` writer - the Interrupt status bit for tell us ASO detectet\n\nASO:Arbitrary Slice Ordering"]
pub type SwAsoDetStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_SLICE_DET_STS` reader - the Interrupt status bit for tell us slice be decoded"]
pub type SwSliceDetStsR = crate::BitReader;
#[doc = "Field `SW_SLICE_DET_STS` writer - the Interrupt status bit for tell us slice be decoded"]
pub type SwSliceDetStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_BSLICE_DET_STS` reader - the Interrupt status bit for tell us B slice be detected"]
pub type SwBsliceDetStsR = crate::BitReader;
#[doc = "Field `SW_BSLICE_DET_STS` writer - the Interrupt status bit for tell us B slice be detected"]
pub type SwBsliceDetStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_ERROR_DET_STS` reader - the Interrupt status bit for tell us error detected\n\nInterrupt status bit input stream error. When high, an error is found\n\nin input data stream decoding. HW will self reset.\n\n(1,2,3,6,48,55,57)"]
pub type SwErrorDetStsR = crate::BitReader;
#[doc = "Field `SW_ERROR_DET_STS` writer - the Interrupt status bit for tell us error detected\n\nInterrupt status bit input stream error. When high, an error is found\n\nin input data stream decoding. HW will self reset.\n\n(1,2,3,6,48,55,57)"]
pub type SwErrorDetStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_TIMEOUT_DET_STS` reader - the Interrupt status bit for tell us timeout detected\n\nAXI in IDLE status too long"]
pub type SwTimeoutDetStsR = crate::BitReader;
#[doc = "Field `SW_TIMEOUT_DET_STS` writer - the Interrupt status bit for tell us timeout detected\n\nAXI in IDLE status too long"]
pub type SwTimeoutDetStsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - the decoder finish interrupt request\n\nafter sw query this interrupt,shoud write 0 to reset."]
    #[inline(always)]
    pub fn sw_dec_irq(&self) -> SwDecIrqR {
        SwDecIrqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the decoder finish interrupt request diable flag"]
    #[inline(always)]
    pub fn sw_dec_irq_dis(&self) -> SwDecIrqDisR {
        SwDecIrqDisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - the Interrupt status bit for tell sw processed a picture"]
    #[inline(always)]
    pub fn sw_dec_rdy_sts(&self) -> SwDecRdyStsR {
        SwDecRdyStsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the Interrupt status bit for tell sw bus have some error"]
    #[inline(always)]
    pub fn sw_pp_bus_sts(&self) -> SwPpBusStsR {
        SwPpBusStsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - the Interrupt status bit for tell input buffer empty"]
    #[inline(always)]
    pub fn sw_buf_emt_sts(&self) -> SwBufEmtStsR {
        SwBufEmtStsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - the Interrupt status bit for tell us ASO detectet\n\nASO:Arbitrary Slice Ordering"]
    #[inline(always)]
    pub fn sw_aso_det_sts(&self) -> SwAsoDetStsR {
        SwAsoDetStsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - the Interrupt status bit for tell us slice be decoded"]
    #[inline(always)]
    pub fn sw_slice_det_sts(&self) -> SwSliceDetStsR {
        SwSliceDetStsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - the Interrupt status bit for tell us B slice be detected"]
    #[inline(always)]
    pub fn sw_bslice_det_sts(&self) -> SwBsliceDetStsR {
        SwBsliceDetStsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - the Interrupt status bit for tell us error detected\n\nInterrupt status bit input stream error. When high, an error is found\n\nin input data stream decoding. HW will self reset.\n\n(1,2,3,6,48,55,57)"]
    #[inline(always)]
    pub fn sw_error_det_sts(&self) -> SwErrorDetStsR {
        SwErrorDetStsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - the Interrupt status bit for tell us timeout detected\n\nAXI in IDLE status too long"]
    #[inline(always)]
    pub fn sw_timeout_det_sts(&self) -> SwTimeoutDetStsR {
        SwTimeoutDetStsR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - the decoder finish interrupt request\n\nafter sw query this interrupt,shoud write 0 to reset."]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_irq(&mut self) -> SwDecIrqW<Swreg55Spec> {
        SwDecIrqW::new(self, 0)
    }
    #[doc = "Bit 1 - the decoder finish interrupt request diable flag"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_irq_dis(&mut self) -> SwDecIrqDisW<Swreg55Spec> {
        SwDecIrqDisW::new(self, 1)
    }
    #[doc = "Bit 4 - the Interrupt status bit for tell sw processed a picture"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_rdy_sts(&mut self) -> SwDecRdyStsW<Swreg55Spec> {
        SwDecRdyStsW::new(self, 4)
    }
    #[doc = "Bit 5 - the Interrupt status bit for tell sw bus have some error"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_bus_sts(&mut self) -> SwPpBusStsW<Swreg55Spec> {
        SwPpBusStsW::new(self, 5)
    }
    #[doc = "Bit 6 - the Interrupt status bit for tell input buffer empty"]
    #[inline(always)]
    #[must_use]
    pub fn sw_buf_emt_sts(&mut self) -> SwBufEmtStsW<Swreg55Spec> {
        SwBufEmtStsW::new(self, 6)
    }
    #[doc = "Bit 8 - the Interrupt status bit for tell us ASO detectet\n\nASO:Arbitrary Slice Ordering"]
    #[inline(always)]
    #[must_use]
    pub fn sw_aso_det_sts(&mut self) -> SwAsoDetStsW<Swreg55Spec> {
        SwAsoDetStsW::new(self, 8)
    }
    #[doc = "Bit 9 - the Interrupt status bit for tell us slice be decoded"]
    #[inline(always)]
    #[must_use]
    pub fn sw_slice_det_sts(&mut self) -> SwSliceDetStsW<Swreg55Spec> {
        SwSliceDetStsW::new(self, 9)
    }
    #[doc = "Bit 10 - the Interrupt status bit for tell us B slice be detected"]
    #[inline(always)]
    #[must_use]
    pub fn sw_bslice_det_sts(&mut self) -> SwBsliceDetStsW<Swreg55Spec> {
        SwBsliceDetStsW::new(self, 10)
    }
    #[doc = "Bit 12 - the Interrupt status bit for tell us error detected\n\nInterrupt status bit input stream error. When high, an error is found\n\nin input data stream decoding. HW will self reset.\n\n(1,2,3,6,48,55,57)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_error_det_sts(&mut self) -> SwErrorDetStsW<Swreg55Spec> {
        SwErrorDetStsW::new(self, 12)
    }
    #[doc = "Bit 13 - the Interrupt status bit for tell us timeout detected\n\nAXI in IDLE status too long"]
    #[inline(always)]
    #[must_use]
    pub fn sw_timeout_det_sts(&mut self) -> SwTimeoutDetStsW<Swreg55Spec> {
        SwTimeoutDetStsW::new(self, 13)
    }
}
#[doc = "decoder int register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg55::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg55::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg55Spec;
impl crate::RegisterSpec for Swreg55Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg55::R`](R) reader structure"]
impl crate::Readable for Swreg55Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg55::W`](W) writer structure"]
impl crate::Writable for Swreg55Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG55 to value 0"]
impl crate::Resettable for Swreg55Spec {
    const RESET_VALUE: u32 = 0;
}
