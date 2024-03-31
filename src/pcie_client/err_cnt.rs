#[doc = "Register `ERR_CNT` reader"]
pub type R = crate::R<ErrCntSpec>;
#[doc = "Register `ERR_CNT` writer"]
pub type W = crate::W<ErrCntSpec>;
#[doc = "Field `FATAL_ERR_CNT` reader - Fatal error counter\n\nFatal error counter, write all one(8'hff) clear the counter."]
pub type FatalErrCntR = crate::FieldReader;
#[doc = "Field `FATAL_ERR_CNT` writer - Fatal error counter\n\nFatal error counter, write all one(8'hff) clear the counter."]
pub type FatalErrCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NFATAL_ERR_CNT` reader - Non-fatal error counter\n\nNon-fatal error counter, write all one(8'hff) clear the counter."]
pub type NfatalErrCntR = crate::FieldReader;
#[doc = "Field `NFATAL_ERR_CNT` writer - Non-fatal error counter\n\nNon-fatal error counter, write all one(8'hff) clear the counter."]
pub type NfatalErrCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CORR_ERR_CNT` reader - Correctable error counter\n\nCorrectable error counter, write all one(8'hff) clear the counter."]
pub type CorrErrCntR = crate::FieldReader;
#[doc = "Field `CORR_ERR_CNT` writer - Correctable error counter\n\nCorrectable error counter, write all one(8'hff) clear the counter."]
pub type CorrErrCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Fatal error counter\n\nFatal error counter, write all one(8'hff) clear the counter."]
    #[inline(always)]
    pub fn fatal_err_cnt(&self) -> FatalErrCntR {
        FatalErrCntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Non-fatal error counter\n\nNon-fatal error counter, write all one(8'hff) clear the counter."]
    #[inline(always)]
    pub fn nfatal_err_cnt(&self) -> NfatalErrCntR {
        NfatalErrCntR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Correctable error counter\n\nCorrectable error counter, write all one(8'hff) clear the counter."]
    #[inline(always)]
    pub fn corr_err_cnt(&self) -> CorrErrCntR {
        CorrErrCntR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fatal error counter\n\nFatal error counter, write all one(8'hff) clear the counter."]
    #[inline(always)]
    #[must_use]
    pub fn fatal_err_cnt(&mut self) -> FatalErrCntW<ErrCntSpec> {
        FatalErrCntW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Non-fatal error counter\n\nNon-fatal error counter, write all one(8'hff) clear the counter."]
    #[inline(always)]
    #[must_use]
    pub fn nfatal_err_cnt(&mut self) -> NfatalErrCntW<ErrCntSpec> {
        NfatalErrCntW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Correctable error counter\n\nCorrectable error counter, write all one(8'hff) clear the counter."]
    #[inline(always)]
    #[must_use]
    pub fn corr_err_cnt(&mut self) -> CorrErrCntW<ErrCntSpec> {
        CorrErrCntW::new(self, 16)
    }
}
#[doc = "Error counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrCntSpec;
impl crate::RegisterSpec for ErrCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_cnt::R`](R) reader structure"]
impl crate::Readable for ErrCntSpec {}
#[doc = "`write(|w| ..)` method takes [`err_cnt::W`](W) writer structure"]
impl crate::Writable for ErrCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x00ff_ffff;
}
#[doc = "`reset()` method sets ERR_CNT to value 0"]
impl crate::Resettable for ErrCntSpec {
    const RESET_VALUE: u32 = 0;
}
