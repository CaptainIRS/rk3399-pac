#[doc = "Register `PI_REG_54` reader"]
pub type R = crate::R<PiReg54Spec>;
#[doc = "Register `PI_REG_54` writer"]
pub type W = crate::W<PiReg54Spec>;
#[doc = "Field `PI_SWLVL_WR_SLICE_1` writer - Indicates software leveling write command in WDQ training for data slice 1."]
pub type PiSwlvlWrSlice1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SWLVL_RD_SLICE_1` writer - Indicates software leveling read command in WDQ training for data slice 1."]
pub type PiSwlvlRdSlice1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SW_WDQLVL_RESP_1` reader - Indicates WDQ leveling response for data slice 1."]
pub type PiSwWdqlvlResp1R = crate::FieldReader;
#[doc = "Field `PI_SWLVL_WR_SLICE_2` writer - Indicates software leveling write command in WDQ training for data slice 2."]
pub type PiSwlvlWrSlice2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:17 - Indicates WDQ leveling response for data slice 1."]
    #[inline(always)]
    pub fn pi_sw_wdqlvl_resp_1(&self) -> PiSwWdqlvlResp1R {
        PiSwWdqlvlResp1R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates software leveling write command in WDQ training for data slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_wr_slice_1(&mut self) -> PiSwlvlWrSlice1W<PiReg54Spec> {
        PiSwlvlWrSlice1W::new(self, 0)
    }
    #[doc = "Bit 8 - Indicates software leveling read command in WDQ training for data slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_rd_slice_1(&mut self) -> PiSwlvlRdSlice1W<PiReg54Spec> {
        PiSwlvlRdSlice1W::new(self, 8)
    }
    #[doc = "Bit 24 - Indicates software leveling write command in WDQ training for data slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_wr_slice_2(&mut self) -> PiSwlvlWrSlice2W<PiReg54Spec> {
        PiSwlvlWrSlice2W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 54\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_54::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_54::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg54Spec;
impl crate::RegisterSpec for PiReg54Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_54::R`](R) reader structure"]
impl crate::Readable for PiReg54Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_54::W`](W) writer structure"]
impl crate::Writable for PiReg54Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_54 to value 0"]
impl crate::Resettable for PiReg54Spec {
    const RESET_VALUE: u32 = 0;
}
