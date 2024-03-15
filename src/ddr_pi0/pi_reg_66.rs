#[doc = "Register `PI_REG_66` reader"]
pub type R = crate::R<PiReg66Spec>;
#[doc = "Register `PI_REG_66` writer"]
pub type W = crate::W<PiReg66Spec>;
#[doc = "Field `PI_WRLVL_STROBE_NUM` reader - Defines the write leveling strobe number in LPDDR4."]
pub type PiWrlvlStrobeNumR = crate::FieldReader;
#[doc = "Field `PI_WRLVL_STROBE_NUM` writer - Defines the write leveling strobe number in LPDDR4."]
pub type PiWrlvlStrobeNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TODTL_2CMD_F0` reader - Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTodtl2cmdF0R = crate::FieldReader;
#[doc = "Field `PI_TODTL_2CMD_F0` writer - Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTodtl2cmdF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4 - Defines the write leveling strobe number in LPDDR4."]
    #[inline(always)]
    pub fn pi_wrlvl_strobe_num(&self) -> PiWrlvlStrobeNumR {
        PiWrlvlStrobeNumR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 24:31 - Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_todtl_2cmd_f0(&self) -> PiTodtl2cmdF0R {
        PiTodtl2cmdF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Defines the write leveling strobe number in LPDDR4."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_strobe_num(&mut self) -> PiWrlvlStrobeNumW<PiReg66Spec> {
        PiWrlvlStrobeNumW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Defines the DRAM delay from an ODT de-assertion to the next non-write, non-read command. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_todtl_2cmd_f0(&mut self) -> PiTodtl2cmdF0W<PiReg66Spec> {
        PiTodtl2cmdF0W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 66\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_66::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_66::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg66Spec;
impl crate::RegisterSpec for PiReg66Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_66::R`](R) reader structure"]
impl crate::Readable for PiReg66Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_66::W`](W) writer structure"]
impl crate::Writable for PiReg66Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_66 to value 0"]
impl crate::Resettable for PiReg66Spec {
    const RESET_VALUE: u32 = 0;
}
