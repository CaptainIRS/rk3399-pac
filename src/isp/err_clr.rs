#[doc = "Register `ERR_CLR` writer"]
pub type W = crate::W<ErrClrSpec>;
#[doc = "Field `inform_size_err_clr` writer - size error is cleared"]
pub type InformSizeErrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `is_size_err_clr` writer - size error is cleared"]
pub type IsSizeErrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `outform_size_err_clr` writer - size error is cleared"]
pub type OutformSizeErrClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - size error is cleared"]
    #[inline(always)]
    #[must_use]
    pub fn inform_size_err_clr(&mut self) -> InformSizeErrClrW<ErrClrSpec> {
        InformSizeErrClrW::new(self, 0)
    }
    #[doc = "Bit 1 - size error is cleared"]
    #[inline(always)]
    #[must_use]
    pub fn is_size_err_clr(&mut self) -> IsSizeErrClrW<ErrClrSpec> {
        IsSizeErrClrW::new(self, 1)
    }
    #[doc = "Bit 2 - size error is cleared"]
    #[inline(always)]
    #[must_use]
    pub fn outform_size_err_clr(&mut self) -> OutformSizeErrClrW<ErrClrSpec> {
        OutformSizeErrClrW::new(self, 2)
    }
}
#[doc = "ISP error clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrClrSpec;
impl crate::RegisterSpec for ErrClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`err_clr::W`](W) writer structure"]
impl crate::Writable for ErrClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERR_CLR to value 0"]
impl crate::Resettable for ErrClrSpec {
    const RESET_VALUE: u32 = 0;
}
