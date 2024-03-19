#[doc = "Register `DP_RESERV2` reader"]
pub type R = crate::R<DpReserv2Spec>;
#[doc = "Register `DP_RESERV2` writer"]
pub type W = crate::W<DpReserv2Spec>;
#[doc = "ch0,2 swing and pre emphasis control \n\nfor firmware \n\nwhen tx_common&lt;7>=1\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch0Ch2SwingEmpCtrl {
    #[doc = "0: swing0 pre emphasis 0 dB"]
    B0000 = 0,
    #[doc = "1: swing1 pre emphasis 0 dB"]
    B0001 = 1,
    #[doc = "2: swing2 pre emphasis 0 dB"]
    B0010 = 2,
    #[doc = "3: swing3 pre emphasis 0 dB"]
    B0011 = 3,
    #[doc = "4: swing0 pre emphasis 3.5 dB"]
    B0100 = 4,
    #[doc = "5: swing1 pre emphasis 3.5 dB"]
    B0101 = 5,
    #[doc = "6: swing2 pre emphasis 3.5 dB"]
    B0110 = 6,
    #[doc = "8: swing0 pre emphasis 6 dB"]
    B1000 = 8,
    #[doc = "9: swing1 pre emphasis 6 dB"]
    B1001 = 9,
    #[doc = "12: swing0 pre emphasis 9.5 dB others : swing0 pre emphasis 9.5 dB"]
    B1100 = 12,
}
impl From<Ch0Ch2SwingEmpCtrl> for u8 {
    #[inline(always)]
    fn from(variant: Ch0Ch2SwingEmpCtrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch0Ch2SwingEmpCtrl {
    type Ux = u8;
}
#[doc = "Field `CH0_CH2_SWING_EMP_CTRL` reader - ch0,2 swing and pre emphasis control \n\nfor firmware \n\nwhen tx_common&lt;7>=1"]
pub type Ch0Ch2SwingEmpCtrlR = crate::FieldReader<Ch0Ch2SwingEmpCtrl>;
impl Ch0Ch2SwingEmpCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch0Ch2SwingEmpCtrl> {
        match self.bits {
            0 => Some(Ch0Ch2SwingEmpCtrl::B0000),
            1 => Some(Ch0Ch2SwingEmpCtrl::B0001),
            2 => Some(Ch0Ch2SwingEmpCtrl::B0010),
            3 => Some(Ch0Ch2SwingEmpCtrl::B0011),
            4 => Some(Ch0Ch2SwingEmpCtrl::B0100),
            5 => Some(Ch0Ch2SwingEmpCtrl::B0101),
            6 => Some(Ch0Ch2SwingEmpCtrl::B0110),
            8 => Some(Ch0Ch2SwingEmpCtrl::B1000),
            9 => Some(Ch0Ch2SwingEmpCtrl::B1001),
            12 => Some(Ch0Ch2SwingEmpCtrl::B1100),
            _ => None,
        }
    }
    #[doc = "swing0 pre emphasis 0 dB"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Ch0Ch2SwingEmpCtrl::B0000
    }
    #[doc = "swing1 pre emphasis 0 dB"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Ch0Ch2SwingEmpCtrl::B0001
    }
    #[doc = "swing2 pre emphasis 0 dB"]
    #[inline(always)]
    pub fn is_b0010(&self) -> bool {
        *self == Ch0Ch2SwingEmpCtrl::B0010
    }
    #[doc = "swing3 pre emphasis 0 dB"]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == Ch0Ch2SwingEmpCtrl::B0011
    }
    #[doc = "swing0 pre emphasis 3.5 dB"]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == Ch0Ch2SwingEmpCtrl::B0100
    }
    #[doc = "swing1 pre emphasis 3.5 dB"]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == Ch0Ch2SwingEmpCtrl::B0101
    }
    #[doc = "swing2 pre emphasis 3.5 dB"]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == Ch0Ch2SwingEmpCtrl::B0110
    }
    #[doc = "swing0 pre emphasis 6 dB"]
    #[inline(always)]
    pub fn is_b1000(&self) -> bool {
        *self == Ch0Ch2SwingEmpCtrl::B1000
    }
    #[doc = "swing1 pre emphasis 6 dB"]
    #[inline(always)]
    pub fn is_b1001(&self) -> bool {
        *self == Ch0Ch2SwingEmpCtrl::B1001
    }
    #[doc = "swing0 pre emphasis 9.5 dB others : swing0 pre emphasis 9.5 dB"]
    #[inline(always)]
    pub fn is_b1100(&self) -> bool {
        *self == Ch0Ch2SwingEmpCtrl::B1100
    }
}
#[doc = "Field `CH0_CH2_SWING_EMP_CTRL` writer - ch0,2 swing and pre emphasis control \n\nfor firmware \n\nwhen tx_common&lt;7>=1"]
pub type Ch0Ch2SwingEmpCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ch0Ch2SwingEmpCtrl>;
impl<'a, REG> Ch0Ch2SwingEmpCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "swing0 pre emphasis 0 dB"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0Ch2SwingEmpCtrl::B0000)
    }
    #[doc = "swing1 pre emphasis 0 dB"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0Ch2SwingEmpCtrl::B0001)
    }
    #[doc = "swing2 pre emphasis 0 dB"]
    #[inline(always)]
    pub fn b0010(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0Ch2SwingEmpCtrl::B0010)
    }
    #[doc = "swing3 pre emphasis 0 dB"]
    #[inline(always)]
    pub fn b0011(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0Ch2SwingEmpCtrl::B0011)
    }
    #[doc = "swing0 pre emphasis 3.5 dB"]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0Ch2SwingEmpCtrl::B0100)
    }
    #[doc = "swing1 pre emphasis 3.5 dB"]
    #[inline(always)]
    pub fn b0101(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0Ch2SwingEmpCtrl::B0101)
    }
    #[doc = "swing2 pre emphasis 3.5 dB"]
    #[inline(always)]
    pub fn b0110(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0Ch2SwingEmpCtrl::B0110)
    }
    #[doc = "swing0 pre emphasis 6 dB"]
    #[inline(always)]
    pub fn b1000(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0Ch2SwingEmpCtrl::B1000)
    }
    #[doc = "swing1 pre emphasis 6 dB"]
    #[inline(always)]
    pub fn b1001(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0Ch2SwingEmpCtrl::B1001)
    }
    #[doc = "swing0 pre emphasis 9.5 dB others : swing0 pre emphasis 9.5 dB"]
    #[inline(always)]
    pub fn b1100(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0Ch2SwingEmpCtrl::B1100)
    }
}
#[doc = "ch1,3 swing and pre emphasis control \n\nfor firmware \n\nwhen tx_common&lt;7>=1\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch1Ch3SwingEmpCtrl {
    #[doc = "0: swing0 pre emphasis 0 dB"]
    B0000 = 0,
    #[doc = "1: swing1 pre emphasis 0 dB"]
    B0001 = 1,
    #[doc = "2: swing2 pre emphasis 0 dB"]
    B0010 = 2,
    #[doc = "3: swing3 pre emphasis 0 dB"]
    B0011 = 3,
    #[doc = "4: swing0 pre emphasis 3.5 dB"]
    B0100 = 4,
    #[doc = "5: swing1 pre emphasis 3.5 dB"]
    B0101 = 5,
    #[doc = "6: swing2 pre emphasis 3.5 dB"]
    B0110 = 6,
    #[doc = "8: swing0 pre emphasis 6 dB"]
    B1000 = 8,
    #[doc = "9: swing1 pre emphasis 6 dB"]
    B1001 = 9,
    #[doc = "12: swing0 pre emphasis 9.5 dB others : swing0 pre emphasis 9.5 dB"]
    B1100 = 12,
}
impl From<Ch1Ch3SwingEmpCtrl> for u8 {
    #[inline(always)]
    fn from(variant: Ch1Ch3SwingEmpCtrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch1Ch3SwingEmpCtrl {
    type Ux = u8;
}
#[doc = "Field `CH1_CH3_SWING_EMP_CTRL` reader - ch1,3 swing and pre emphasis control \n\nfor firmware \n\nwhen tx_common&lt;7>=1"]
pub type Ch1Ch3SwingEmpCtrlR = crate::FieldReader<Ch1Ch3SwingEmpCtrl>;
impl Ch1Ch3SwingEmpCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch1Ch3SwingEmpCtrl> {
        match self.bits {
            0 => Some(Ch1Ch3SwingEmpCtrl::B0000),
            1 => Some(Ch1Ch3SwingEmpCtrl::B0001),
            2 => Some(Ch1Ch3SwingEmpCtrl::B0010),
            3 => Some(Ch1Ch3SwingEmpCtrl::B0011),
            4 => Some(Ch1Ch3SwingEmpCtrl::B0100),
            5 => Some(Ch1Ch3SwingEmpCtrl::B0101),
            6 => Some(Ch1Ch3SwingEmpCtrl::B0110),
            8 => Some(Ch1Ch3SwingEmpCtrl::B1000),
            9 => Some(Ch1Ch3SwingEmpCtrl::B1001),
            12 => Some(Ch1Ch3SwingEmpCtrl::B1100),
            _ => None,
        }
    }
    #[doc = "swing0 pre emphasis 0 dB"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Ch1Ch3SwingEmpCtrl::B0000
    }
    #[doc = "swing1 pre emphasis 0 dB"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Ch1Ch3SwingEmpCtrl::B0001
    }
    #[doc = "swing2 pre emphasis 0 dB"]
    #[inline(always)]
    pub fn is_b0010(&self) -> bool {
        *self == Ch1Ch3SwingEmpCtrl::B0010
    }
    #[doc = "swing3 pre emphasis 0 dB"]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == Ch1Ch3SwingEmpCtrl::B0011
    }
    #[doc = "swing0 pre emphasis 3.5 dB"]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == Ch1Ch3SwingEmpCtrl::B0100
    }
    #[doc = "swing1 pre emphasis 3.5 dB"]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == Ch1Ch3SwingEmpCtrl::B0101
    }
    #[doc = "swing2 pre emphasis 3.5 dB"]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == Ch1Ch3SwingEmpCtrl::B0110
    }
    #[doc = "swing0 pre emphasis 6 dB"]
    #[inline(always)]
    pub fn is_b1000(&self) -> bool {
        *self == Ch1Ch3SwingEmpCtrl::B1000
    }
    #[doc = "swing1 pre emphasis 6 dB"]
    #[inline(always)]
    pub fn is_b1001(&self) -> bool {
        *self == Ch1Ch3SwingEmpCtrl::B1001
    }
    #[doc = "swing0 pre emphasis 9.5 dB others : swing0 pre emphasis 9.5 dB"]
    #[inline(always)]
    pub fn is_b1100(&self) -> bool {
        *self == Ch1Ch3SwingEmpCtrl::B1100
    }
}
#[doc = "Field `CH1_CH3_SWING_EMP_CTRL` writer - ch1,3 swing and pre emphasis control \n\nfor firmware \n\nwhen tx_common&lt;7>=1"]
pub type Ch1Ch3SwingEmpCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ch1Ch3SwingEmpCtrl>;
impl<'a, REG> Ch1Ch3SwingEmpCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "swing0 pre emphasis 0 dB"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1Ch3SwingEmpCtrl::B0000)
    }
    #[doc = "swing1 pre emphasis 0 dB"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1Ch3SwingEmpCtrl::B0001)
    }
    #[doc = "swing2 pre emphasis 0 dB"]
    #[inline(always)]
    pub fn b0010(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1Ch3SwingEmpCtrl::B0010)
    }
    #[doc = "swing3 pre emphasis 0 dB"]
    #[inline(always)]
    pub fn b0011(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1Ch3SwingEmpCtrl::B0011)
    }
    #[doc = "swing0 pre emphasis 3.5 dB"]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1Ch3SwingEmpCtrl::B0100)
    }
    #[doc = "swing1 pre emphasis 3.5 dB"]
    #[inline(always)]
    pub fn b0101(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1Ch3SwingEmpCtrl::B0101)
    }
    #[doc = "swing2 pre emphasis 3.5 dB"]
    #[inline(always)]
    pub fn b0110(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1Ch3SwingEmpCtrl::B0110)
    }
    #[doc = "swing0 pre emphasis 6 dB"]
    #[inline(always)]
    pub fn b1000(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1Ch3SwingEmpCtrl::B1000)
    }
    #[doc = "swing1 pre emphasis 6 dB"]
    #[inline(always)]
    pub fn b1001(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1Ch3SwingEmpCtrl::B1001)
    }
    #[doc = "swing0 pre emphasis 9.5 dB others : swing0 pre emphasis 9.5 dB"]
    #[inline(always)]
    pub fn b1100(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1Ch3SwingEmpCtrl::B1100)
    }
}
impl R {
    #[doc = "Bits 0:3 - ch0,2 swing and pre emphasis control \n\nfor firmware \n\nwhen tx_common&lt;7>=1"]
    #[inline(always)]
    pub fn ch0_ch2_swing_emp_ctrl(&self) -> Ch0Ch2SwingEmpCtrlR {
        Ch0Ch2SwingEmpCtrlR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ch1,3 swing and pre emphasis control \n\nfor firmware \n\nwhen tx_common&lt;7>=1"]
    #[inline(always)]
    pub fn ch1_ch3_swing_emp_ctrl(&self) -> Ch1Ch3SwingEmpCtrlR {
        Ch1Ch3SwingEmpCtrlR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ch0,2 swing and pre emphasis control \n\nfor firmware \n\nwhen tx_common&lt;7>=1"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_ch2_swing_emp_ctrl(&mut self) -> Ch0Ch2SwingEmpCtrlW<DpReserv2Spec> {
        Ch0Ch2SwingEmpCtrlW::new(self, 0)
    }
    #[doc = "Bits 4:7 - ch1,3 swing and pre emphasis control \n\nfor firmware \n\nwhen tx_common&lt;7>=1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_ch3_swing_emp_ctrl(&mut self) -> Ch1Ch3SwingEmpCtrlW<DpReserv2Spec> {
        Ch1Ch3SwingEmpCtrlW::new(self, 4)
    }
}
#[doc = "RESERVD2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_reserv2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_reserv2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpReserv2Spec;
impl crate::RegisterSpec for DpReserv2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_reserv2::R`](R) reader structure"]
impl crate::Readable for DpReserv2Spec {}
#[doc = "`write(|w| ..)` method takes [`dp_reserv2::W`](W) writer structure"]
impl crate::Writable for DpReserv2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets DP_RESERV2 to value 0x11"]
impl crate::Resettable for DpReserv2Spec {
    const RESET_VALUE: u32 = 0x11;
}
