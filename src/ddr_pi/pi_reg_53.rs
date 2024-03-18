#[doc = "Register `PI_REG_53` reader"]
pub type R = crate::R<PiReg53Spec>;
#[doc = "Register `PI_REG_53` writer"]
pub type W = crate::W<PiReg53Spec>;
#[doc = "Field `PI_SWLVL_EXIT` writer - Indicates user request to exit software leveling. Set to 1 to exit."]
pub type PiSwlvlExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SWLVL_WR_SLICE_0` writer - Indicates software leveling write command in WDQ training for data slice 0."]
pub type PiSwlvlWrSlice0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SWLVL_RD_SLICE_0` writer - Indicates software leveling read command in WDQ training for data slice 0."]
pub type PiSwlvlRdSlice0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SW_WDQLVL_RESP_0` reader - Indicates WDQ leveling response for data slice 0."]
pub type PiSwWdqlvlResp0R = crate::FieldReader;
impl R {
    #[doc = "Bits 24:25 - Indicates WDQ leveling response for data slice 0."]
    #[inline(always)]
    pub fn pi_sw_wdqlvl_resp_0(&self) -> PiSwWdqlvlResp0R {
        PiSwWdqlvlResp0R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates user request to exit software leveling. Set to 1 to exit."]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_exit(&mut self) -> PiSwlvlExitW<PiReg53Spec> {
        PiSwlvlExitW::new(self, 0)
    }
    #[doc = "Bit 8 - Indicates software leveling write command in WDQ training for data slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_wr_slice_0(&mut self) -> PiSwlvlWrSlice0W<PiReg53Spec> {
        PiSwlvlWrSlice0W::new(self, 8)
    }
    #[doc = "Bit 16 - Indicates software leveling read command in WDQ training for data slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_rd_slice_0(&mut self) -> PiSwlvlRdSlice0W<PiReg53Spec> {
        PiSwlvlRdSlice0W::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_53::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_53::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg53Spec;
impl crate::RegisterSpec for PiReg53Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_53::R`](R) reader structure"]
impl crate::Readable for PiReg53Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_53::W`](W) writer structure"]
impl crate::Writable for PiReg53Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_53 to value 0"]
impl crate::Resettable for PiReg53Spec {
    const RESET_VALUE: u32 = 0;
}
