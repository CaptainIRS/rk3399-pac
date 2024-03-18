#[doc = "Register `PI_REG_68` reader"]
pub type R = crate::R<PiReg68Spec>;
#[doc = "Register `PI_REG_68` writer"]
pub type W = crate::W<PiReg68Spec>;
#[doc = "Field `PI_ODT_EN_F2` reader - Enables support of DRAM ODT. When enabled, PI asserts and de-asserts ODT output to DRAM as needed. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiOdtEnF2R = crate::BitReader;
#[doc = "Field `PI_ODT_EN_F2` writer - Enables support of DRAM ODT. When enabled, PI asserts and de-asserts ODT output to DRAM as needed. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiOdtEnF2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_TODTH_WR` reader - Defines the DRAM minimum ODT high time after an ODT assertion for a write command."]
pub type PiTodthWrR = crate::FieldReader;
#[doc = "Field `PI_TODTH_WR` writer - Defines the DRAM minimum ODT high time after an ODT assertion for a write command."]
pub type PiTodthWrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TODTH_RD` reader - Defines the DRAM minimum ODT high time after an ODT assertion for a read command."]
pub type PiTodthRdR = crate::FieldReader;
#[doc = "Field `PI_TODTH_RD` writer - Defines the DRAM minimum ODT high time after an ODT assertion for a read command."]
pub type PiTodthRdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_ODT_RD_MAP_CS0` reader - Determines the chip(s) that have termination when a read occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a read."]
pub type PiOdtRdMapCs0R = crate::FieldReader;
#[doc = "Field `PI_ODT_RD_MAP_CS0` writer - Determines the chip(s) that have termination when a read occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a read."]
pub type PiOdtRdMapCs0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Enables support of DRAM ODT. When enabled, PI asserts and de-asserts ODT output to DRAM as needed. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_odt_en_f2(&self) -> PiOdtEnF2R {
        PiOdtEnF2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Defines the DRAM minimum ODT high time after an ODT assertion for a write command."]
    #[inline(always)]
    pub fn pi_todth_wr(&self) -> PiTodthWrR {
        PiTodthWrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Defines the DRAM minimum ODT high time after an ODT assertion for a read command."]
    #[inline(always)]
    pub fn pi_todth_rd(&self) -> PiTodthRdR {
        PiTodthRdR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - Determines the chip(s) that have termination when a read occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a read."]
    #[inline(always)]
    pub fn pi_odt_rd_map_cs0(&self) -> PiOdtRdMapCs0R {
        PiOdtRdMapCs0R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables support of DRAM ODT. When enabled, PI asserts and de-asserts ODT output to DRAM as needed. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odt_en_f2(&mut self) -> PiOdtEnF2W<PiReg68Spec> {
        PiOdtEnF2W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Defines the DRAM minimum ODT high time after an ODT assertion for a write command."]
    #[inline(always)]
    #[must_use]
    pub fn pi_todth_wr(&mut self) -> PiTodthWrW<PiReg68Spec> {
        PiTodthWrW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Defines the DRAM minimum ODT high time after an ODT assertion for a read command."]
    #[inline(always)]
    #[must_use]
    pub fn pi_todth_rd(&mut self) -> PiTodthRdW<PiReg68Spec> {
        PiTodthRdW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Determines the chip(s) that have termination when a read occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a read."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odt_rd_map_cs0(&mut self) -> PiOdtRdMapCs0W<PiReg68Spec> {
        PiOdtRdMapCs0W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 68\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_68::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_68::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg68Spec;
impl crate::RegisterSpec for PiReg68Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_68::R`](R) reader structure"]
impl crate::Readable for PiReg68Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_68::W`](W) writer structure"]
impl crate::Writable for PiReg68Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_68 to value 0"]
impl crate::Resettable for PiReg68Spec {
    const RESET_VALUE: u32 = 0;
}