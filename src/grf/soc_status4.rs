#[doc = "Register `SOC_STATUS4` reader"]
pub type R = crate::R<SocStatus4Spec>;
#[doc = "Register `SOC_STATUS4` writer"]
pub type W = crate::W<SocStatus4Spec>;
#[doc = "Field `DDR_MONITOR` reader - ddr_monitor\\[31:0\\]
status bit"]
pub type DdrMonitorR = crate::FieldReader<u32>;
#[doc = "Field `DDR_MONITOR` writer - ddr_monitor\\[31:0\\]
status bit"]
pub type DdrMonitorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ddr_monitor\\[31:0\\]
status bit"]
    #[inline(always)]
    pub fn ddr_monitor(&self) -> DdrMonitorR {
        DdrMonitorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ddr_monitor\\[31:0\\]
status bit"]
    #[inline(always)]
    #[must_use]
    pub fn ddr_monitor(&mut self) -> DdrMonitorW<SocStatus4Spec> {
        DdrMonitorW::new(self, 0)
    }
}
#[doc = "SOC status register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_status4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_status4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocStatus4Spec;
impl crate::RegisterSpec for SocStatus4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc_status4::R`](R) reader structure"]
impl crate::Readable for SocStatus4Spec {}
#[doc = "`write(|w| ..)` method takes [`soc_status4::W`](W) writer structure"]
impl crate::Writable for SocStatus4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOC_STATUS4 to value 0"]
impl crate::Resettable for SocStatus4Spec {
    const RESET_VALUE: u32 = 0;
}
