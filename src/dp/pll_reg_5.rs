#[doc = "Register `PLL_REG_5` reader"]
pub type R = crate::R<PllReg5Spec>;
#[doc = "Register `PLL_REG_5` writer"]
pub type W = crate::W<PllReg5Spec>;
#[doc = "option to control charge pump input voltage for 0.95V master regulator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChgPumpInputCtrlOp {
    #[doc = "0: 1.8V"]
    B0 = 0,
    #[doc = "1: 1.8V"]
    B1 = 1,
}
impl From<ChgPumpInputCtrlOp> for bool {
    #[inline(always)]
    fn from(variant: ChgPumpInputCtrlOp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHG_PUMP_INPUT_CTRL_OP` reader - option to control charge pump input voltage for 0.95V master regulator"]
pub type ChgPumpInputCtrlOpR = crate::BitReader<ChgPumpInputCtrlOp>;
impl ChgPumpInputCtrlOpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChgPumpInputCtrlOp {
        match self.bits {
            false => ChgPumpInputCtrlOp::B0,
            true => ChgPumpInputCtrlOp::B1,
        }
    }
    #[doc = "1.8V"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ChgPumpInputCtrlOp::B0
    }
    #[doc = "1.8V"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ChgPumpInputCtrlOp::B1
    }
}
#[doc = "Field `CHG_PUMP_INPUT_CTRL_OP` writer - option to control charge pump input voltage for 0.95V master regulator"]
pub type ChgPumpInputCtrlOpW<'a, REG> = crate::BitWriter1C<'a, REG, ChgPumpInputCtrlOp>;
impl<'a, REG> ChgPumpInputCtrlOpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1.8V"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ChgPumpInputCtrlOp::B0)
    }
    #[doc = "1.8V"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ChgPumpInputCtrlOp::B1)
    }
}
#[doc = "control charge pump input voltage for 0.95V master regulator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ChgPumpInputCtrl {
    #[doc = "0: 1.4V"]
    B00 = 0,
    #[doc = "1: 1.4V"]
    B01 = 1,
    #[doc = "2: 1.4V"]
    B10 = 2,
    #[doc = "3: 1.4V"]
    B11 = 3,
}
impl From<ChgPumpInputCtrl> for u8 {
    #[inline(always)]
    fn from(variant: ChgPumpInputCtrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ChgPumpInputCtrl {
    type Ux = u8;
}
#[doc = "Field `CHG_PUMP_INPUT_CTRL` reader - control charge pump input voltage for 0.95V master regulator"]
pub type ChgPumpInputCtrlR = crate::FieldReader<ChgPumpInputCtrl>;
impl ChgPumpInputCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChgPumpInputCtrl {
        match self.bits {
            0 => ChgPumpInputCtrl::B00,
            1 => ChgPumpInputCtrl::B01,
            2 => ChgPumpInputCtrl::B10,
            3 => ChgPumpInputCtrl::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "1.4V"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ChgPumpInputCtrl::B00
    }
    #[doc = "1.4V"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ChgPumpInputCtrl::B01
    }
    #[doc = "1.4V"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ChgPumpInputCtrl::B10
    }
    #[doc = "1.4V"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == ChgPumpInputCtrl::B11
    }
}
#[doc = "Field `CHG_PUMP_INPUT_CTRL` writer - control charge pump input voltage for 0.95V master regulator"]
pub type ChgPumpInputCtrlW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ChgPumpInputCtrl>;
impl<'a, REG> ChgPumpInputCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.4V"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ChgPumpInputCtrl::B00)
    }
    #[doc = "1.4V"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ChgPumpInputCtrl::B01)
    }
    #[doc = "1.4V"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ChgPumpInputCtrl::B10)
    }
    #[doc = "1.4V"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(ChgPumpInputCtrl::B11)
    }
}
#[doc = "slave standby current select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StandbyCurrentSel {
    #[doc = "1: keep 300uA standby current (default)"]
    B1 = 1,
    #[doc = "0: keep 300uA standby current (default)"]
    B0 = 0,
}
impl From<StandbyCurrentSel> for bool {
    #[inline(always)]
    fn from(variant: StandbyCurrentSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STANDBY_CURRENT_SEL` reader - slave standby current select"]
pub type StandbyCurrentSelR = crate::BitReader<StandbyCurrentSel>;
impl StandbyCurrentSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StandbyCurrentSel {
        match self.bits {
            true => StandbyCurrentSel::B1,
            false => StandbyCurrentSel::B0,
        }
    }
    #[doc = "keep 300uA standby current (default)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == StandbyCurrentSel::B1
    }
    #[doc = "keep 300uA standby current (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == StandbyCurrentSel::B0
    }
}
#[doc = "Field `STANDBY_CURRENT_SEL` writer - slave standby current select"]
pub type StandbyCurrentSelW<'a, REG> = crate::BitWriter1C<'a, REG, StandbyCurrentSel>;
impl<'a, REG> StandbyCurrentSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "keep 300uA standby current (default)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(StandbyCurrentSel::B1)
    }
    #[doc = "keep 300uA standby current (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(StandbyCurrentSel::B0)
    }
}
#[doc = "Field `REGULATOR_V_SEL` reader - slave regulator output voltage select 000 900V 001 0.925V 010 0.950V(default) 011 0.975V 100 1.000V 101 1.025V 110 1.050V"]
pub type RegulatorVSelR = crate::FieldReader;
#[doc = "Field `REGULATOR_V_SEL` writer - slave regulator output voltage select 000 900V 001 0.925V 010 0.950V(default) 011 0.975V 100 1.000V 101 1.025V 110 1.050V"]
pub type RegulatorVSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - option to control charge pump input voltage for 0.95V master regulator"]
    #[inline(always)]
    pub fn chg_pump_input_ctrl_op(&self) -> ChgPumpInputCtrlOpR {
        ChgPumpInputCtrlOpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - control charge pump input voltage for 0.95V master regulator"]
    #[inline(always)]
    pub fn chg_pump_input_ctrl(&self) -> ChgPumpInputCtrlR {
        ChgPumpInputCtrlR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - slave standby current select"]
    #[inline(always)]
    pub fn standby_current_sel(&self) -> StandbyCurrentSelR {
        StandbyCurrentSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - slave regulator output voltage select 000 900V 001 0.925V 010 0.950V(default) 011 0.975V 100 1.000V 101 1.025V 110 1.050V"]
    #[inline(always)]
    pub fn regulator_v_sel(&self) -> RegulatorVSelR {
        RegulatorVSelR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - option to control charge pump input voltage for 0.95V master regulator"]
    #[inline(always)]
    #[must_use]
    pub fn chg_pump_input_ctrl_op(&mut self) -> ChgPumpInputCtrlOpW<PllReg5Spec> {
        ChgPumpInputCtrlOpW::new(self, 0)
    }
    #[doc = "Bits 1:2 - control charge pump input voltage for 0.95V master regulator"]
    #[inline(always)]
    #[must_use]
    pub fn chg_pump_input_ctrl(&mut self) -> ChgPumpInputCtrlW<PllReg5Spec> {
        ChgPumpInputCtrlW::new(self, 1)
    }
    #[doc = "Bit 3 - slave standby current select"]
    #[inline(always)]
    #[must_use]
    pub fn standby_current_sel(&mut self) -> StandbyCurrentSelW<PllReg5Spec> {
        StandbyCurrentSelW::new(self, 3)
    }
    #[doc = "Bits 4:6 - slave regulator output voltage select 000 900V 001 0.925V 010 0.950V(default) 011 0.975V 100 1.000V 101 1.025V 110 1.050V"]
    #[inline(always)]
    #[must_use]
    pub fn regulator_v_sel(&mut self) -> RegulatorVSelW<PllReg5Spec> {
        RegulatorVSelW::new(self, 4)
    }
}
#[doc = "Pll_control_5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_reg_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_reg_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllReg5Spec;
impl crate::RegisterSpec for PllReg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_reg_5::R`](R) reader structure"]
impl crate::Readable for PllReg5Spec {}
#[doc = "`write(|w| ..)` method takes [`pll_reg_5::W`](W) writer structure"]
impl crate::Writable for PllReg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets PLL_REG_5 to value 0"]
impl crate::Resettable for PllReg5Spec {
    const RESET_VALUE: u32 = 0;
}
