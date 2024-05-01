#[doc = "Register `SWREG56` reader"]
pub type R = crate::R<Swreg56Spec>;
#[doc = "Register `SWREG56` writer"]
pub type W = crate::W<Swreg56Spec>;
#[doc = "Field `SW_DEC_AXI_ID_RD` reader - AXI Read ID\n\nif you config 0/5,will modify as 1 by hw"]
pub type SwDecAxiIdRdR = crate::FieldReader;
#[doc = "Field `SW_DEC_AXI_ID_RD` writer - AXI Read ID\n\nif you config 0/5,will modify as 1 by hw"]
pub type SwDecAxiIdRdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_DEC_AXI_ID_WR` reader - AXI Write ID\n\nif you config 0,will modify as 1 by hw"]
pub type SwDecAxiIdWrR = crate::FieldReader;
#[doc = "Field `SW_DEC_AXI_ID_WR` writer - AXI Write ID\n\nif you config 0,will modify as 1 by hw"]
pub type SwDecAxiIdWrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_DEC_MAX_BURLEN` reader - the max burst length can be used by axi bus\n\nrange : 1-16"]
pub type SwDecMaxBurlenR = crate::FieldReader;
#[doc = "Field `SW_DEC_MAX_BURLEN` writer - the max burst length can be used by axi bus\n\nrange : 1-16"]
pub type SwDecMaxBurlenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "the parallel or serial mode for axi read and write\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwBusPosSel {
    #[doc = "0: serial"]
    B0 = 0,
    #[doc = "1: parallel"]
    B1 = 1,
}
impl From<SwBusPosSel> for bool {
    #[inline(always)]
    fn from(variant: SwBusPosSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_BUS_POS_SEL` reader - the parallel or serial mode for axi read and write"]
pub type SwBusPosSelR = crate::BitReader<SwBusPosSel>;
impl SwBusPosSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwBusPosSel {
        match self.bits {
            false => SwBusPosSel::B0,
            true => SwBusPosSel::B1,
        }
    }
    #[doc = "serial"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwBusPosSel::B0
    }
    #[doc = "parallel"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwBusPosSel::B1
    }
}
#[doc = "Field `SW_BUS_POS_SEL` writer - the parallel or serial mode for axi read and write"]
pub type SwBusPosSelW<'a, REG> = crate::BitWriter<'a, REG, SwBusPosSel>;
impl<'a, REG> SwBusPosSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "serial"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwBusPosSel::B0)
    }
    #[doc = "parallel"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwBusPosSel::B1)
    }
}
#[doc = "Field `SW_DEC_DATA_DISCD_EN` reader - enable for Data discard\n\nthe fixed burst length will be used ,and the more read datas will be\n\nauto discarded by hw"]
pub type SwDecDataDiscdEnR = crate::BitReader;
#[doc = "Field `SW_DEC_DATA_DISCD_EN` writer - enable for Data discard\n\nthe fixed burst length will be used ,and the more read datas will be\n\nauto discarded by hw"]
pub type SwDecDataDiscdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "axi signals selected for encoder or decoder\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwAxiSel {
    #[doc = "0: auto sel for encoder or decoder"]
    B0 = 0,
    #[doc = "1: sel decoder (only used in the middle decoder frame to set bus_dec_en to 0)"]
    B1 = 1,
}
impl From<SwAxiSel> for bool {
    #[inline(always)]
    fn from(variant: SwAxiSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_AXI_SEL` reader - axi signals selected for encoder or decoder"]
pub type SwAxiSelR = crate::BitReader<SwAxiSel>;
impl SwAxiSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwAxiSel {
        match self.bits {
            false => SwAxiSel::B0,
            true => SwAxiSel::B1,
        }
    }
    #[doc = "auto sel for encoder or decoder"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwAxiSel::B0
    }
    #[doc = "sel decoder (only used in the middle decoder frame to set bus_dec_en to 0)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwAxiSel::B1
    }
}
#[doc = "Field `SW_AXI_SEL` writer - axi signals selected for encoder or decoder"]
pub type SwAxiSelW<'a, REG> = crate::BitWriter<'a, REG, SwAxiSel>;
impl<'a, REG> SwAxiSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "auto sel for encoder or decoder"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwAxiSel::B0)
    }
    #[doc = "sel decoder (only used in the middle decoder frame to set bus_dec_en to 0)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwAxiSel::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - AXI Read ID\n\nif you config 0/5,will modify as 1 by hw"]
    #[inline(always)]
    pub fn sw_dec_axi_id_rd(&self) -> SwDecAxiIdRdR {
        SwDecAxiIdRdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - AXI Write ID\n\nif you config 0,will modify as 1 by hw"]
    #[inline(always)]
    pub fn sw_dec_axi_id_wr(&self) -> SwDecAxiIdWrR {
        SwDecAxiIdWrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - the max burst length can be used by axi bus\n\nrange : 1-16"]
    #[inline(always)]
    pub fn sw_dec_max_burlen(&self) -> SwDecMaxBurlenR {
        SwDecMaxBurlenR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - the parallel or serial mode for axi read and write"]
    #[inline(always)]
    pub fn sw_bus_pos_sel(&self) -> SwBusPosSelR {
        SwBusPosSelR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - enable for Data discard\n\nthe fixed burst length will be used ,and the more read datas will be\n\nauto discarded by hw"]
    #[inline(always)]
    pub fn sw_dec_data_discd_en(&self) -> SwDecDataDiscdEnR {
        SwDecDataDiscdEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - axi signals selected for encoder or decoder"]
    #[inline(always)]
    pub fn sw_axi_sel(&self) -> SwAxiSelR {
        SwAxiSelR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - AXI Read ID\n\nif you config 0/5,will modify as 1 by hw"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_axi_id_rd(&mut self) -> SwDecAxiIdRdW<Swreg56Spec> {
        SwDecAxiIdRdW::new(self, 0)
    }
    #[doc = "Bits 8:15 - AXI Write ID\n\nif you config 0,will modify as 1 by hw"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_axi_id_wr(&mut self) -> SwDecAxiIdWrW<Swreg56Spec> {
        SwDecAxiIdWrW::new(self, 8)
    }
    #[doc = "Bits 16:20 - the max burst length can be used by axi bus\n\nrange : 1-16"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_max_burlen(&mut self) -> SwDecMaxBurlenW<Swreg56Spec> {
        SwDecMaxBurlenW::new(self, 16)
    }
    #[doc = "Bit 21 - the parallel or serial mode for axi read and write"]
    #[inline(always)]
    #[must_use]
    pub fn sw_bus_pos_sel(&mut self) -> SwBusPosSelW<Swreg56Spec> {
        SwBusPosSelW::new(self, 21)
    }
    #[doc = "Bit 22 - enable for Data discard\n\nthe fixed burst length will be used ,and the more read datas will be\n\nauto discarded by hw"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_data_discd_en(&mut self) -> SwDecDataDiscdEnW<Swreg56Spec> {
        SwDecDataDiscdEnW::new(self, 22)
    }
    #[doc = "Bit 23 - axi signals selected for encoder or decoder"]
    #[inline(always)]
    #[must_use]
    pub fn sw_axi_sel(&mut self) -> SwAxiSelW<Swreg56Spec> {
        SwAxiSelW::new(self, 23)
    }
}
#[doc = "axi ctrl for decoder\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg56::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg56::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg56Spec;
impl crate::RegisterSpec for Swreg56Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg56::R`](R) reader structure"]
impl crate::Readable for Swreg56Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg56::W`](W) writer structure"]
impl crate::Writable for Swreg56Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG56 to value 0x0020_0101"]
impl crate::Resettable for Swreg56Spec {
    const RESET_VALUE: u32 = 0x0020_0101;
}
