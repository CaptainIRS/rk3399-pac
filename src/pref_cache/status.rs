#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `CMD_BUSY` reader - set when the cache is busy handling commands"]
pub type CmdBusyR = crate::BitReader;
#[doc = "Field `CMD_BUSY` writer - set when the cache is busy handling commands"]
pub type CmdBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_BUSY` reader - set when the cache is busy handling data"]
pub type DataBusyR = crate::BitReader;
#[doc = "Field `DATA_BUSY` writer - set when the cache is busy handling data"]
pub type DataBusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - set when the cache is busy handling commands"]
    #[inline(always)]
    pub fn cmd_busy(&self) -> CmdBusyR {
        CmdBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - set when the cache is busy handling data"]
    #[inline(always)]
    pub fn data_busy(&self) -> DataBusyR {
        DataBusyR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - set when the cache is busy handling commands"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_busy(&mut self) -> CmdBusyW<StatusSpec> {
        CmdBusyW::new(self, 0)
    }
    #[doc = "Bit 1 - set when the cache is busy handling data"]
    #[inline(always)]
    #[must_use]
    pub fn data_busy(&mut self) -> DataBusyW<StatusSpec> {
        DataBusyW::new(self, 1)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
