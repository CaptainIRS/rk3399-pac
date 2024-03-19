#[doc = "Register `DDR_PI_REG_83` reader"]
pub type R = crate::R<DdrPiReg83Spec>;
#[doc = "Register `DDR_PI_REG_83` writer"]
pub type W = crate::W<DdrPiReg83Spec>;
#[doc = "Field `PI_RDLVL_GATE_INTERVAL` reader - The number of long count sequences that are counted between automatic gate training commands."]
pub type PiRdlvlGateIntervalR = crate::FieldReader<u16>;
#[doc = "Field `PI_RDLVL_GATE_INTERVAL` writer - The number of long count sequences that are counted between automatic gate training commands."]
pub type PiRdlvlGateIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_RDLVL_PATTERN_START` reader - Defines the start pattern in read leveling."]
pub type PiRdlvlPatternStartR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_PATTERN_START` writer - Defines the start pattern in read leveling."]
pub type PiRdlvlPatternStartW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_RDLVL_PATTERN_NUM` reader - Defines the number of patterns that are supported in read leveling."]
pub type PiRdlvlPatternNumR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_PATTERN_NUM` writer - Defines the number of patterns that are supported in read leveling."]
pub type PiRdlvlPatternNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - The number of long count sequences that are counted between automatic gate training commands."]
    #[inline(always)]
    pub fn pi_rdlvl_gate_interval(&self) -> PiRdlvlGateIntervalR {
        PiRdlvlGateIntervalR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Defines the start pattern in read leveling."]
    #[inline(always)]
    pub fn pi_rdlvl_pattern_start(&self) -> PiRdlvlPatternStartR {
        PiRdlvlPatternStartR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Defines the number of patterns that are supported in read leveling."]
    #[inline(always)]
    pub fn pi_rdlvl_pattern_num(&self) -> PiRdlvlPatternNumR {
        PiRdlvlPatternNumR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - The number of long count sequences that are counted between automatic gate training commands."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_gate_interval(&mut self) -> PiRdlvlGateIntervalW<DdrPiReg83Spec> {
        PiRdlvlGateIntervalW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Defines the start pattern in read leveling."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_pattern_start(&mut self) -> PiRdlvlPatternStartW<DdrPiReg83Spec> {
        PiRdlvlPatternStartW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Defines the number of patterns that are supported in read leveling."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_pattern_num(&mut self) -> PiRdlvlPatternNumW<DdrPiReg83Spec> {
        PiRdlvlPatternNumW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 83\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_83::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_83::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg83Spec;
impl crate::RegisterSpec for DdrPiReg83Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_83::R`](R) reader structure"]
impl crate::Readable for DdrPiReg83Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_83::W`](W) writer structure"]
impl crate::Writable for DdrPiReg83Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_83 to value 0"]
impl crate::Resettable for DdrPiReg83Spec {
    const RESET_VALUE: u32 = 0;
}
