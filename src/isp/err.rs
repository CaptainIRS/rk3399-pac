#[doc = "Register `ERR` reader"]
pub type R = crate::R<ErrSpec>;
#[doc = "Field `inform_size_err` reader - size error is generated in inform submodule"]
pub type InformSizeErrR = crate::BitReader;
#[doc = "Field `is_size_err` reader - size error is generated in image stabilization\n\nsubmodule"]
pub type IsSizeErrR = crate::BitReader;
#[doc = "Field `outform_size_err` reader - size error is generated in outmux submodule"]
pub type OutformSizeErrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - size error is generated in inform submodule"]
    #[inline(always)]
    pub fn inform_size_err(&self) -> InformSizeErrR {
        InformSizeErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - size error is generated in image stabilization\n\nsubmodule"]
    #[inline(always)]
    pub fn is_size_err(&self) -> IsSizeErrR {
        IsSizeErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - size error is generated in outmux submodule"]
    #[inline(always)]
    pub fn outform_size_err(&self) -> OutformSizeErrR {
        OutformSizeErrR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "ISP error register\n\nNote: For debug purposes the ISP_ERR und ISP_ERR_CLR are implemented. For the case \n\nwhen a PIC_SIZE_ERR interrupt is signaled the SW is able to see in which submodule this error \n\nis generated. Writing to the ISP_ERR_CLR register clears this bit. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrSpec;
impl crate::RegisterSpec for ErrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err::R`](R) reader structure"]
impl crate::Readable for ErrSpec {}
#[doc = "`reset()` method sets ERR to value 0"]
impl crate::Resettable for ErrSpec {
    const RESET_VALUE: u32 = 0;
}
