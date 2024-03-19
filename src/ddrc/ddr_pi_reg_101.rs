#[doc = "Register `DDR_PI_REG_101` reader"]
pub type R = crate::R<DdrPiReg101Spec>;
#[doc = "Register `DDR_PI_REG_101` writer"]
pub type W = crate::W<DdrPiReg101Spec>;
#[doc = "Field `PI_CALVL_INTERVAL` reader - Indicates the number of long count sequences that is counted\n\nbetween automatic CA training commands."]
pub type PiCalvlIntervalR = crate::FieldReader<u16>;
#[doc = "Field `PI_CALVL_INTERVAL` writer - Indicates the number of long count sequences that is counted\n\nbetween automatic CA training commands."]
pub type PiCalvlIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_TCACKEL` reader - Indicates DRAM TCACKEL value in cycles."]
pub type PiTcackelR = crate::FieldReader;
#[doc = "Field `PI_TCACKEL` writer - Indicates DRAM TCACKEL value in cycles."]
pub type PiTcackelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TCAMRD` reader - Indicates DRAM TCAMRD value in cycles."]
pub type PiTcamrdR = crate::FieldReader;
#[doc = "Field `PI_TCAMRD` writer - Indicates DRAM TCAMRD value in cycles."]
pub type PiTcamrdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:15 - Indicates the number of long count sequences that is counted\n\nbetween automatic CA training commands."]
    #[inline(always)]
    pub fn pi_calvl_interval(&self) -> PiCalvlIntervalR {
        PiCalvlIntervalR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Indicates DRAM TCACKEL value in cycles."]
    #[inline(always)]
    pub fn pi_tcackel(&self) -> PiTcackelR {
        PiTcackelR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:29 - Indicates DRAM TCAMRD value in cycles."]
    #[inline(always)]
    pub fn pi_tcamrd(&self) -> PiTcamrdR {
        PiTcamrdR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicates the number of long count sequences that is counted\n\nbetween automatic CA training commands."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_interval(&mut self) -> PiCalvlIntervalW<DdrPiReg101Spec> {
        PiCalvlIntervalW::new(self, 0)
    }
    #[doc = "Bits 16:20 - Indicates DRAM TCACKEL value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcackel(&mut self) -> PiTcackelW<DdrPiReg101Spec> {
        PiTcackelW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Indicates DRAM TCAMRD value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcamrd(&mut self) -> PiTcamrdW<DdrPiReg101Spec> {
        PiTcamrdW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 101\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_101::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_101::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg101Spec;
impl crate::RegisterSpec for DdrPiReg101Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_101::R`](R) reader structure"]
impl crate::Readable for DdrPiReg101Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_101::W`](W) writer structure"]
impl crate::Writable for DdrPiReg101Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_101 to value 0"]
impl crate::Resettable for DdrPiReg101Spec {
    const RESET_VALUE: u32 = 0;
}
