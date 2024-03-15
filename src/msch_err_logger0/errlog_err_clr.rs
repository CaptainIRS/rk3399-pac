#[doc = "Register `ERRLOG_ErrClr` reader"]
pub type R = crate::R<ErrlogErrClrSpec>;
#[doc = "Register `ERRLOG_ErrClr` writer"]
pub type W = crate::W<ErrlogErrClrSpec>;
#[doc = "Field `ERRCLR` reader - When set to 1, clears register ErrVld. Reading ErrClr always returns 0."]
pub type ErrclrR = crate::BitReader;
#[doc = "Field `ERRCLR` writer - When set to 1, clears register ErrVld. Reading ErrClr always returns 0."]
pub type ErrclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set to 1, clears register ErrVld. Reading ErrClr always returns 0."]
    #[inline(always)]
    pub fn errclr(&self) -> ErrclrR {
        ErrclrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set to 1, clears register ErrVld. Reading ErrClr always returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn errclr(&mut self) -> ErrclrW<ErrlogErrClrSpec> {
        ErrclrW::new(self, 0)
    }
}
#[doc = "Error interrupt status clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_err_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`errlog_err_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrlogErrClrSpec;
impl crate::RegisterSpec for ErrlogErrClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errlog_err_clr::R`](R) reader structure"]
impl crate::Readable for ErrlogErrClrSpec {}
#[doc = "`write(|w| ..)` method takes [`errlog_err_clr::W`](W) writer structure"]
impl crate::Writable for ErrlogErrClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERRLOG_ErrClr to value 0"]
impl crate::Resettable for ErrlogErrClrSpec {
    const RESET_VALUE: u32 = 0;
}
