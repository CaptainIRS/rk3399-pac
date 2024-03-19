#[doc = "Register `DP_BIAS` reader"]
pub type R = crate::R<DpBiasSpec>;
#[doc = "Register `DP_BIAS` writer"]
pub type W = crate::W<DpBiasSpec>;
#[doc = "Resistor tune for band gap TC \n\ncontrol\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DpResistorTuneBgCtrl {
    #[doc = "0: 25uV/°C"]
    B00 = 0,
    #[doc = "1: 10uV/°C"]
    B01 = 1,
    #[doc = "2: -10uV/°C"]
    B10 = 2,
    #[doc = "3: -25uV/°C"]
    B11 = 3,
}
impl From<DpResistorTuneBgCtrl> for u8 {
    #[inline(always)]
    fn from(variant: DpResistorTuneBgCtrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DpResistorTuneBgCtrl {
    type Ux = u8;
}
#[doc = "Field `DP_RESISTOR_TUNE_BG_CTRL` reader - Resistor tune for band gap TC \n\ncontrol"]
pub type DpResistorTuneBgCtrlR = crate::FieldReader<DpResistorTuneBgCtrl>;
impl DpResistorTuneBgCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DpResistorTuneBgCtrl {
        match self.bits {
            0 => DpResistorTuneBgCtrl::B00,
            1 => DpResistorTuneBgCtrl::B01,
            2 => DpResistorTuneBgCtrl::B10,
            3 => DpResistorTuneBgCtrl::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "25uV/°C"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == DpResistorTuneBgCtrl::B00
    }
    #[doc = "10uV/°C"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == DpResistorTuneBgCtrl::B01
    }
    #[doc = "-10uV/°C"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == DpResistorTuneBgCtrl::B10
    }
    #[doc = "-25uV/°C"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == DpResistorTuneBgCtrl::B11
    }
}
#[doc = "Field `DP_RESISTOR_TUNE_BG_CTRL` writer - Resistor tune for band gap TC \n\ncontrol"]
pub type DpResistorTuneBgCtrlW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DpResistorTuneBgCtrl>;
impl<'a, REG> DpResistorTuneBgCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "25uV/°C"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(DpResistorTuneBgCtrl::B00)
    }
    #[doc = "10uV/°C"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(DpResistorTuneBgCtrl::B01)
    }
    #[doc = "-10uV/°C"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(DpResistorTuneBgCtrl::B10)
    }
    #[doc = "-25uV/°C"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(DpResistorTuneBgCtrl::B11)
    }
}
#[doc = "Select band gap\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpBgSel {
    #[doc = "0: sel Register"]
    B0 = 0,
    #[doc = "1: sel Band gap"]
    B1 = 1,
}
impl From<DpBgSel> for bool {
    #[inline(always)]
    fn from(variant: DpBgSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DP_BG_SEL` reader - Select band gap"]
pub type DpBgSelR = crate::BitReader<DpBgSel>;
impl DpBgSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DpBgSel {
        match self.bits {
            false => DpBgSel::B0,
            true => DpBgSel::B1,
        }
    }
    #[doc = "sel Register"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DpBgSel::B0
    }
    #[doc = "sel Band gap"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DpBgSel::B1
    }
}
#[doc = "Field `DP_BG_SEL` writer - Select band gap"]
pub type DpBgSelW<'a, REG> = crate::BitWriter<'a, REG, DpBgSel>;
impl<'a, REG> DpBgSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "sel Register"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DpBgSel::B0)
    }
    #[doc = "sel Band gap"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DpBgSel::B1)
    }
}
#[doc = "Band gap start up current control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpDbCureentCtrl {
    #[doc = "0: balance"]
    B0 = 0,
    #[doc = "1: unbalance"]
    B1 = 1,
}
impl From<DpDbCureentCtrl> for bool {
    #[inline(always)]
    fn from(variant: DpDbCureentCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DP_DB_CUREENT_CTRL` reader - Band gap start up current control"]
pub type DpDbCureentCtrlR = crate::BitReader<DpDbCureentCtrl>;
impl DpDbCureentCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DpDbCureentCtrl {
        match self.bits {
            false => DpDbCureentCtrl::B0,
            true => DpDbCureentCtrl::B1,
        }
    }
    #[doc = "balance"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DpDbCureentCtrl::B0
    }
    #[doc = "unbalance"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DpDbCureentCtrl::B1
    }
}
#[doc = "Field `DP_DB_CUREENT_CTRL` writer - Band gap start up current control"]
pub type DpDbCureentCtrlW<'a, REG> = crate::BitWriter1C<'a, REG, DpDbCureentCtrl>;
impl<'a, REG> DpDbCureentCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "balance"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DpDbCureentCtrl::B0)
    }
    #[doc = "unbalance"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DpDbCureentCtrl::B1)
    }
}
#[doc = "Select band gap out\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DpBgOutSel {
    #[doc = "0: 0.56V"]
    B000 = 0,
    #[doc = "4: 0.6V(default)"]
    B100 = 4,
    #[doc = "7: 0.63V"]
    B111 = 7,
}
impl From<DpBgOutSel> for u8 {
    #[inline(always)]
    fn from(variant: DpBgOutSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DpBgOutSel {
    type Ux = u8;
}
#[doc = "Field `DP_BG_OUT_SEL` reader - Select band gap out"]
pub type DpBgOutSelR = crate::FieldReader<DpBgOutSel>;
impl DpBgOutSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DpBgOutSel> {
        match self.bits {
            0 => Some(DpBgOutSel::B000),
            4 => Some(DpBgOutSel::B100),
            7 => Some(DpBgOutSel::B111),
            _ => None,
        }
    }
    #[doc = "0.56V"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == DpBgOutSel::B000
    }
    #[doc = "0.6V(default)"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == DpBgOutSel::B100
    }
    #[doc = "0.63V"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == DpBgOutSel::B111
    }
}
#[doc = "Field `DP_BG_OUT_SEL` writer - Select band gap out"]
pub type DpBgOutSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, DpBgOutSel>;
impl<'a, REG> DpBgOutSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.56V"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(DpBgOutSel::B000)
    }
    #[doc = "0.6V(default)"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(DpBgOutSel::B100)
    }
    #[doc = "0.63V"]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(DpBgOutSel::B111)
    }
}
impl R {
    #[doc = "Bits 0:1 - Resistor tune for band gap TC \n\ncontrol"]
    #[inline(always)]
    pub fn dp_resistor_tune_bg_ctrl(&self) -> DpResistorTuneBgCtrlR {
        DpResistorTuneBgCtrlR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Select band gap"]
    #[inline(always)]
    pub fn dp_bg_sel(&self) -> DpBgSelR {
        DpBgSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Band gap start up current control"]
    #[inline(always)]
    pub fn dp_db_cureent_ctrl(&self) -> DpDbCureentCtrlR {
        DpDbCureentCtrlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Select band gap out"]
    #[inline(always)]
    pub fn dp_bg_out_sel(&self) -> DpBgOutSelR {
        DpBgOutSelR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Resistor tune for band gap TC \n\ncontrol"]
    #[inline(always)]
    #[must_use]
    pub fn dp_resistor_tune_bg_ctrl(&mut self) -> DpResistorTuneBgCtrlW<DpBiasSpec> {
        DpResistorTuneBgCtrlW::new(self, 0)
    }
    #[doc = "Bit 2 - Select band gap"]
    #[inline(always)]
    #[must_use]
    pub fn dp_bg_sel(&mut self) -> DpBgSelW<DpBiasSpec> {
        DpBgSelW::new(self, 2)
    }
    #[doc = "Bit 3 - Band gap start up current control"]
    #[inline(always)]
    #[must_use]
    pub fn dp_db_cureent_ctrl(&mut self) -> DpDbCureentCtrlW<DpBiasSpec> {
        DpDbCureentCtrlW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Select band gap out"]
    #[inline(always)]
    #[must_use]
    pub fn dp_bg_out_sel(&mut self) -> DpBgOutSelW<DpBiasSpec> {
        DpBgOutSelW::new(self, 4)
    }
}
#[doc = "Bias control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_bias::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_bias::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpBiasSpec;
impl crate::RegisterSpec for DpBiasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_bias::R`](R) reader structure"]
impl crate::Readable for DpBiasSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_bias::W`](W) writer structure"]
impl crate::Writable for DpBiasSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0b;
}
#[doc = "`reset()` method sets DP_BIAS to value 0x34"]
impl crate::Resettable for DpBiasSpec {
    const RESET_VALUE: u32 = 0x34;
}
