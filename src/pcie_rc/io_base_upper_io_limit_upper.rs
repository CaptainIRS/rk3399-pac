#[doc = "Register `IO_BASE_UPPER_IO_LIMIT_UPPER` reader"]
pub type R = crate::R<IoBaseUpperIoLimitUpperSpec>;
#[doc = "Field `IBRU` reader - IO Base Register Upper \\[IBRU\\]
This field can be read and written from the local management bus if 32bit IO BAR is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
pub type IbruR = crate::FieldReader<u16>;
#[doc = "Field `ILR` reader - IO Limit Register Upper \\[ILR\\]
This field can be read and written from the local management bus if 32bit IO BAR is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
pub type IlrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IO Base Register Upper \\[IBRU\\]
This field can be read and written from the local management bus if 32bit IO BAR is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
    #[inline(always)]
    pub fn ibru(&self) -> IbruR {
        IbruR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IO Limit Register Upper \\[ILR\\]
This field can be read and written from the local management bus if 32bit IO BAR is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
    #[inline(always)]
    pub fn ilr(&self) -> IlrR {
        IlrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "IO Base Upper, IO Limit Upper This field can be read and written from the local management bus if 32bit IO BAR is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_base_upper_io_limit_upper::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoBaseUpperIoLimitUpperSpec;
impl crate::RegisterSpec for IoBaseUpperIoLimitUpperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_base_upper_io_limit_upper::R`](R) reader structure"]
impl crate::Readable for IoBaseUpperIoLimitUpperSpec {}
#[doc = "`reset()` method sets IO_BASE_UPPER_IO_LIMIT_UPPER to value 0"]
impl crate::Resettable for IoBaseUpperIoLimitUpperSpec {
    const RESET_VALUE: u32 = 0;
}
