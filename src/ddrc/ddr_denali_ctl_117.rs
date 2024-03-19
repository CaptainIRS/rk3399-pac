#[doc = "Register `DDR_DENALI_CTL_117` reader"]
pub type R = crate::R<DdrDenaliCtl117Spec>;
#[doc = "Register `DDR_DENALI_CTL_117` writer"]
pub type W = crate::W<DdrDenaliCtl117Spec>;
#[doc = "Field `WRITE_MODEREG` reader - Write memory mode register data to the DRAMs. Bits (7:0) define the memory mode register number if bit (23) is set, bits (15:8) define the chip select if bit (24) is clear, bits (23:16) define which memory mode register/s to write, bit (24) defines whether all chip selects will be written, and bit (25) triggers the write."]
pub type WriteModeregR = crate::FieldReader<u32>;
#[doc = "Field `WRITE_MODEREG` writer - Write memory mode register data to the DRAMs. Bits (7:0) define the memory mode register number if bit (23) is set, bits (15:8) define the chip select if bit (24) is clear, bits (23:16) define which memory mode register/s to write, bit (24) defines whether all chip selects will be written, and bit (25) triggers the write."]
pub type WriteModeregW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:26 - Write memory mode register data to the DRAMs. Bits (7:0) define the memory mode register number if bit (23) is set, bits (15:8) define the chip select if bit (24) is clear, bits (23:16) define which memory mode register/s to write, bit (24) defines whether all chip selects will be written, and bit (25) triggers the write."]
    #[inline(always)]
    pub fn write_modereg(&self) -> WriteModeregR {
        WriteModeregR::new(self.bits & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:26 - Write memory mode register data to the DRAMs. Bits (7:0) define the memory mode register number if bit (23) is set, bits (15:8) define the chip select if bit (24) is clear, bits (23:16) define which memory mode register/s to write, bit (24) defines whether all chip selects will be written, and bit (25) triggers the write."]
    #[inline(always)]
    #[must_use]
    pub fn write_modereg(&mut self) -> WriteModeregW<DdrDenaliCtl117Spec> {
        WriteModeregW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_117::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_117::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl117Spec;
impl crate::RegisterSpec for DdrDenaliCtl117Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_117::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl117Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_117::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl117Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_117 to value 0"]
impl crate::Resettable for DdrDenaliCtl117Spec {
    const RESET_VALUE: u32 = 0;
}
