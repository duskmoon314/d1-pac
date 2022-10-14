#[doc = "Register `ac_adc_drc_ctrl` reader"]
pub struct R(crate::R<AC_ADC_DRC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_ADC_DRC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_ADC_DRC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_ADC_DRC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ac_adc_drc_ctrl` writer"]
pub struct W(crate::W<AC_ADC_DRC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_ADC_DRC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<AC_ADC_DRC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_ADC_DRC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adc_drc_et_en` reader - DRC ET enable\n\nWhen the bit is disabled, Ke and OPE parameter is unused."]
pub type ADC_DRC_ET_EN_R = crate::BitReader<ADC_DRC_ET_EN_A>;
#[doc = "DRC ET enable\n\nWhen the bit is disabled, Ke and OPE parameter is unused.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_DRC_ET_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADC_DRC_ET_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_DRC_ET_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_DRC_ET_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DRC_ET_EN_A {
        match self.bits {
            false => ADC_DRC_ET_EN_A::DISABLED,
            true => ADC_DRC_ET_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_DRC_ET_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_DRC_ET_EN_A::ENABLED
    }
}
#[doc = "Field `adc_drc_et_en` writer - DRC ET enable\n\nWhen the bit is disabled, Ke and OPE parameter is unused."]
pub type ADC_DRC_ET_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_ADC_DRC_CTRL_SPEC, ADC_DRC_ET_EN_A, O>;
impl<'a, const O: u8> ADC_DRC_ET_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_DRC_ET_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_DRC_ET_EN_A::ENABLED)
    }
}
#[doc = "Field `adc_drc_lt_en` reader - DRC LT enable\n\nWhen the bit is disabled, Kl and OPL parameter is unused."]
pub type ADC_DRC_LT_EN_R = crate::BitReader<ADC_DRC_LT_EN_A>;
#[doc = "DRC LT enable\n\nWhen the bit is disabled, Kl and OPL parameter is unused.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_DRC_LT_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADC_DRC_LT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_DRC_LT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_DRC_LT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DRC_LT_EN_A {
        match self.bits {
            false => ADC_DRC_LT_EN_A::DISABLED,
            true => ADC_DRC_LT_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_DRC_LT_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_DRC_LT_EN_A::ENABLED
    }
}
#[doc = "Field `adc_drc_lt_en` writer - DRC LT enable\n\nWhen the bit is disabled, Kl and OPL parameter is unused."]
pub type ADC_DRC_LT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_ADC_DRC_CTRL_SPEC, ADC_DRC_LT_EN_A, O>;
impl<'a, const O: u8> ADC_DRC_LT_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_DRC_LT_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_DRC_LT_EN_A::ENABLED)
    }
}
#[doc = "Field `adc_drc_delay_func_en` reader - Delay function enable\n\nWhen the bit is disabled, the signal delay time is unused."]
pub type ADC_DRC_DELAY_FUNC_EN_R = crate::BitReader<ADC_DRC_DELAY_FUNC_EN_A>;
#[doc = "Delay function enable\n\nWhen the bit is disabled, the signal delay time is unused.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_DRC_DELAY_FUNC_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADC_DRC_DELAY_FUNC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_DRC_DELAY_FUNC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_DRC_DELAY_FUNC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DRC_DELAY_FUNC_EN_A {
        match self.bits {
            false => ADC_DRC_DELAY_FUNC_EN_A::DISABLED,
            true => ADC_DRC_DELAY_FUNC_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_DRC_DELAY_FUNC_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_DRC_DELAY_FUNC_EN_A::ENABLED
    }
}
#[doc = "Field `adc_drc_delay_func_en` writer - Delay function enable\n\nWhen the bit is disabled, the signal delay time is unused."]
pub type ADC_DRC_DELAY_FUNC_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_ADC_DRC_CTRL_SPEC, ADC_DRC_DELAY_FUNC_EN_A, O>;
impl<'a, const O: u8> ADC_DRC_DELAY_FUNC_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_DRC_DELAY_FUNC_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_DRC_DELAY_FUNC_EN_A::ENABLED)
    }
}
#[doc = "Field `adc_drc_signal_func_sel` reader - Signal function select\n\nWhen the signal function selects the Peak filter, the RMS parameter is unused. (AC_DRC_LRMSHAT, AC_DRC_LRMSLAT, AC_DRC_LRMSHAT, and AC_DRC_LRMSLAT)\n\nWhen the signal function selects the RMS filter, the Peak filter parameter is unused. (AC_DRC_LPFHAT, AC_DRC_LPFLAT, AC_DRC_RPFHAT, AC_DRC_RPFLAT, AC_DRC_LPFHRT, AC_DRC_LPFLRT, AC_DRC_RPFHRT, and AC_DRC_RPFLRT)"]
pub type ADC_DRC_SIGNAL_FUNC_SEL_R = crate::BitReader<ADC_DRC_SIGNAL_FUNC_SEL_A>;
#[doc = "Signal function select\n\nWhen the signal function selects the Peak filter, the RMS parameter is unused. (AC_DRC_LRMSHAT, AC_DRC_LRMSLAT, AC_DRC_LRMSHAT, and AC_DRC_LRMSLAT)\n\nWhen the signal function selects the RMS filter, the Peak filter parameter is unused. (AC_DRC_LPFHAT, AC_DRC_LPFLAT, AC_DRC_RPFHAT, AC_DRC_RPFLAT, AC_DRC_LPFHRT, AC_DRC_LPFLRT, AC_DRC_RPFHRT, and AC_DRC_RPFLRT)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_DRC_SIGNAL_FUNC_SEL_A {
    #[doc = "0: RMS filter"]
    RMS = 0,
    #[doc = "1: Peak filter"]
    P_EAK = 1,
}
impl From<ADC_DRC_SIGNAL_FUNC_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_DRC_SIGNAL_FUNC_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_DRC_SIGNAL_FUNC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DRC_SIGNAL_FUNC_SEL_A {
        match self.bits {
            false => ADC_DRC_SIGNAL_FUNC_SEL_A::RMS,
            true => ADC_DRC_SIGNAL_FUNC_SEL_A::P_EAK,
        }
    }
    #[doc = "Checks if the value of the field is `RMS`"]
    #[inline(always)]
    pub fn is_rms(&self) -> bool {
        *self == ADC_DRC_SIGNAL_FUNC_SEL_A::RMS
    }
    #[doc = "Checks if the value of the field is `P_EAK`"]
    #[inline(always)]
    pub fn is_p_eak(&self) -> bool {
        *self == ADC_DRC_SIGNAL_FUNC_SEL_A::P_EAK
    }
}
#[doc = "Field `adc_drc_signal_func_sel` writer - Signal function select\n\nWhen the signal function selects the Peak filter, the RMS parameter is unused. (AC_DRC_LRMSHAT, AC_DRC_LRMSLAT, AC_DRC_LRMSHAT, and AC_DRC_LRMSLAT)\n\nWhen the signal function selects the RMS filter, the Peak filter parameter is unused. (AC_DRC_LPFHAT, AC_DRC_LPFLAT, AC_DRC_RPFHAT, AC_DRC_RPFLAT, AC_DRC_LPFHRT, AC_DRC_LPFLRT, AC_DRC_RPFHRT, and AC_DRC_RPFLRT)"]
pub type ADC_DRC_SIGNAL_FUNC_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_ADC_DRC_CTRL_SPEC, ADC_DRC_SIGNAL_FUNC_SEL_A, O>;
impl<'a, const O: u8> ADC_DRC_SIGNAL_FUNC_SEL_W<'a, O> {
    #[doc = "RMS filter"]
    #[inline(always)]
    pub fn rms(self) -> &'a mut W {
        self.variant(ADC_DRC_SIGNAL_FUNC_SEL_A::RMS)
    }
    #[doc = "Peak filter"]
    #[inline(always)]
    pub fn p_eak(self) -> &'a mut W {
        self.variant(ADC_DRC_SIGNAL_FUNC_SEL_A::P_EAK)
    }
}
#[doc = "Field `adc_drc_detect_noise_en` reader - Control the DRC to detect noise when ET is enabled"]
pub type ADC_DRC_DETECT_NOISE_EN_R = crate::BitReader<ADC_DRC_DETECT_NOISE_EN_A>;
#[doc = "Control the DRC to detect noise when ET is enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_DRC_DETECT_NOISE_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADC_DRC_DETECT_NOISE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_DRC_DETECT_NOISE_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_DRC_DETECT_NOISE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DRC_DETECT_NOISE_EN_A {
        match self.bits {
            false => ADC_DRC_DETECT_NOISE_EN_A::DISABLED,
            true => ADC_DRC_DETECT_NOISE_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_DRC_DETECT_NOISE_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_DRC_DETECT_NOISE_EN_A::ENABLED
    }
}
#[doc = "Field `adc_drc_detect_noise_en` writer - Control the DRC to detect noise when ET is enabled"]
pub type ADC_DRC_DETECT_NOISE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_ADC_DRC_CTRL_SPEC, ADC_DRC_DETECT_NOISE_EN_A, O>;
impl<'a, const O: u8> ADC_DRC_DETECT_NOISE_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_DRC_DETECT_NOISE_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_DRC_DETECT_NOISE_EN_A::ENABLED)
    }
}
#[doc = "Field `adc_drc_gain_min_limit_en` reader - DRC gain min limit enable\n\nWhen this fuction is enabled, it will overwrite the noise detect function."]
pub type ADC_DRC_GAIN_MIN_LIMIT_EN_R = crate::BitReader<ADC_DRC_GAIN_MIN_LIMIT_EN_A>;
#[doc = "DRC gain min limit enable\n\nWhen this fuction is enabled, it will overwrite the noise detect function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_DRC_GAIN_MIN_LIMIT_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADC_DRC_GAIN_MIN_LIMIT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_DRC_GAIN_MIN_LIMIT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_DRC_GAIN_MIN_LIMIT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DRC_GAIN_MIN_LIMIT_EN_A {
        match self.bits {
            false => ADC_DRC_GAIN_MIN_LIMIT_EN_A::DISABLED,
            true => ADC_DRC_GAIN_MIN_LIMIT_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_DRC_GAIN_MIN_LIMIT_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_DRC_GAIN_MIN_LIMIT_EN_A::ENABLED
    }
}
#[doc = "Field `adc_drc_gain_min_limit_en` writer - DRC gain min limit enable\n\nWhen this fuction is enabled, it will overwrite the noise detect function."]
pub type ADC_DRC_GAIN_MIN_LIMIT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_ADC_DRC_CTRL_SPEC, ADC_DRC_GAIN_MIN_LIMIT_EN_A, O>;
impl<'a, const O: u8> ADC_DRC_GAIN_MIN_LIMIT_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_DRC_GAIN_MIN_LIMIT_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_DRC_GAIN_MIN_LIMIT_EN_A::ENABLED)
    }
}
#[doc = "Field `adc_drc_gain_max_limit_en` reader - DRC gain max limit enable"]
pub type ADC_DRC_GAIN_MAX_LIMIT_EN_R = crate::BitReader<ADC_DRC_GAIN_MAX_LIMIT_EN_A>;
#[doc = "DRC gain max limit enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_DRC_GAIN_MAX_LIMIT_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADC_DRC_GAIN_MAX_LIMIT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_DRC_GAIN_MAX_LIMIT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_DRC_GAIN_MAX_LIMIT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DRC_GAIN_MAX_LIMIT_EN_A {
        match self.bits {
            false => ADC_DRC_GAIN_MAX_LIMIT_EN_A::DISABLED,
            true => ADC_DRC_GAIN_MAX_LIMIT_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_DRC_GAIN_MAX_LIMIT_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_DRC_GAIN_MAX_LIMIT_EN_A::ENABLED
    }
}
#[doc = "Field `adc_drc_gain_max_limit_en` writer - DRC gain max limit enable"]
pub type ADC_DRC_GAIN_MAX_LIMIT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_ADC_DRC_CTRL_SPEC, ADC_DRC_GAIN_MAX_LIMIT_EN_A, O>;
impl<'a, const O: u8> ADC_DRC_GAIN_MAX_LIMIT_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_DRC_GAIN_MAX_LIMIT_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_DRC_GAIN_MAX_LIMIT_EN_A::ENABLED)
    }
}
#[doc = "Field `adc_drc_delay_buf_en` reader - The delay buffer use or not when the DRC is disabled and the DRC buffer data output completely."]
pub type ADC_DRC_DELAY_BUF_EN_R = crate::BitReader<ADC_DRC_DELAY_BUF_EN_A>;
#[doc = "The delay buffer use or not when the DRC is disabled and the DRC buffer data output completely.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_DRC_DELAY_BUF_EN_A {
    #[doc = "0: Do not use the buffer"]
    D_O_NOT_USE_THE_BUFFER = 0,
    #[doc = "1: Use the buffer"]
    U_SE_THE_BUFFER = 1,
}
impl From<ADC_DRC_DELAY_BUF_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_DRC_DELAY_BUF_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_DRC_DELAY_BUF_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DRC_DELAY_BUF_EN_A {
        match self.bits {
            false => ADC_DRC_DELAY_BUF_EN_A::D_O_NOT_USE_THE_BUFFER,
            true => ADC_DRC_DELAY_BUF_EN_A::U_SE_THE_BUFFER,
        }
    }
    #[doc = "Checks if the value of the field is `D_O_NOT_USE_THE_BUFFER`"]
    #[inline(always)]
    pub fn is_d_o_not_use_the_buffer(&self) -> bool {
        *self == ADC_DRC_DELAY_BUF_EN_A::D_O_NOT_USE_THE_BUFFER
    }
    #[doc = "Checks if the value of the field is `U_SE_THE_BUFFER`"]
    #[inline(always)]
    pub fn is_u_se_the_buffer(&self) -> bool {
        *self == ADC_DRC_DELAY_BUF_EN_A::U_SE_THE_BUFFER
    }
}
#[doc = "Field `adc_drc_delay_buf_en` writer - The delay buffer use or not when the DRC is disabled and the DRC buffer data output completely."]
pub type ADC_DRC_DELAY_BUF_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_ADC_DRC_CTRL_SPEC, ADC_DRC_DELAY_BUF_EN_A, O>;
impl<'a, const O: u8> ADC_DRC_DELAY_BUF_EN_W<'a, O> {
    #[doc = "Do not use the buffer"]
    #[inline(always)]
    pub fn d_o_not_use_the_buffer(self) -> &'a mut W {
        self.variant(ADC_DRC_DELAY_BUF_EN_A::D_O_NOT_USE_THE_BUFFER)
    }
    #[doc = "Use the buffer"]
    #[inline(always)]
    pub fn u_se_the_buffer(self) -> &'a mut W {
        self.variant(ADC_DRC_DELAY_BUF_EN_A::U_SE_THE_BUFFER)
    }
}
#[doc = "Field `adc_drc_signal_delay_time_set` reader - Signal delay time setting\n\nDelay time = 8*(n + 1) fs, n < 6'h30\n\nWhen the delay function is disabled, the signal delay time is unused."]
pub type ADC_DRC_SIGNAL_DELAY_TIME_SET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adc_drc_signal_delay_time_set` writer - Signal delay time setting\n\nDelay time = 8*(n + 1) fs, n < 6'h30\n\nWhen the delay function is disabled, the signal delay time is unused."]
pub type ADC_DRC_SIGNAL_DELAY_TIME_SET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AC_ADC_DRC_CTRL_SPEC, u8, u8, 6, O>;
#[doc = "Field `adc_drc_delay_buf_output_state` reader - DRC delay buffer data output state when The DRC delay function is enabled and the DRC function is disabled. After disabled DRC function and this bit goes to 0, the user should write the DRC delay function bit to 0."]
pub type ADC_DRC_DELAY_BUF_OUTPUT_STATE_R = crate::BitReader<ADC_DRC_DELAY_BUF_OUTPUT_STATE_A>;
#[doc = "DRC delay buffer data output state when The DRC delay function is enabled and the DRC function is disabled. After disabled DRC function and this bit goes to 0, the user should write the DRC delay function bit to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_DRC_DELAY_BUF_OUTPUT_STATE_A {
    #[doc = "0: Not completed"]
    NOT_COMPLETED = 0,
    #[doc = "1: Completed"]
    COMPLETED = 1,
}
impl From<ADC_DRC_DELAY_BUF_OUTPUT_STATE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_DRC_DELAY_BUF_OUTPUT_STATE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_DRC_DELAY_BUF_OUTPUT_STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DRC_DELAY_BUF_OUTPUT_STATE_A {
        match self.bits {
            false => ADC_DRC_DELAY_BUF_OUTPUT_STATE_A::NOT_COMPLETED,
            true => ADC_DRC_DELAY_BUF_OUTPUT_STATE_A::COMPLETED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_COMPLETED`"]
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == ADC_DRC_DELAY_BUF_OUTPUT_STATE_A::NOT_COMPLETED
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == ADC_DRC_DELAY_BUF_OUTPUT_STATE_A::COMPLETED
    }
}
impl R {
    #[doc = "Bit 0 - DRC ET enable\n\nWhen the bit is disabled, Ke and OPE parameter is unused."]
    #[inline(always)]
    pub fn adc_drc_et_en(&self) -> ADC_DRC_ET_EN_R {
        ADC_DRC_ET_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DRC LT enable\n\nWhen the bit is disabled, Kl and OPL parameter is unused."]
    #[inline(always)]
    pub fn adc_drc_lt_en(&self) -> ADC_DRC_LT_EN_R {
        ADC_DRC_LT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Delay function enable\n\nWhen the bit is disabled, the signal delay time is unused."]
    #[inline(always)]
    pub fn adc_drc_delay_func_en(&self) -> ADC_DRC_DELAY_FUNC_EN_R {
        ADC_DRC_DELAY_FUNC_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Signal function select\n\nWhen the signal function selects the Peak filter, the RMS parameter is unused. (AC_DRC_LRMSHAT, AC_DRC_LRMSLAT, AC_DRC_LRMSHAT, and AC_DRC_LRMSLAT)\n\nWhen the signal function selects the RMS filter, the Peak filter parameter is unused. (AC_DRC_LPFHAT, AC_DRC_LPFLAT, AC_DRC_RPFHAT, AC_DRC_RPFLAT, AC_DRC_LPFHRT, AC_DRC_LPFLRT, AC_DRC_RPFHRT, and AC_DRC_RPFLRT)"]
    #[inline(always)]
    pub fn adc_drc_signal_func_sel(&self) -> ADC_DRC_SIGNAL_FUNC_SEL_R {
        ADC_DRC_SIGNAL_FUNC_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Control the DRC to detect noise when ET is enabled"]
    #[inline(always)]
    pub fn adc_drc_detect_noise_en(&self) -> ADC_DRC_DETECT_NOISE_EN_R {
        ADC_DRC_DETECT_NOISE_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DRC gain min limit enable\n\nWhen this fuction is enabled, it will overwrite the noise detect function."]
    #[inline(always)]
    pub fn adc_drc_gain_min_limit_en(&self) -> ADC_DRC_GAIN_MIN_LIMIT_EN_R {
        ADC_DRC_GAIN_MIN_LIMIT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DRC gain max limit enable"]
    #[inline(always)]
    pub fn adc_drc_gain_max_limit_en(&self) -> ADC_DRC_GAIN_MAX_LIMIT_EN_R {
        ADC_DRC_GAIN_MAX_LIMIT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The delay buffer use or not when the DRC is disabled and the DRC buffer data output completely."]
    #[inline(always)]
    pub fn adc_drc_delay_buf_en(&self) -> ADC_DRC_DELAY_BUF_EN_R {
        ADC_DRC_DELAY_BUF_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Signal delay time setting\n\nDelay time = 8*(n + 1) fs, n < 6'h30\n\nWhen the delay function is disabled, the signal delay time is unused."]
    #[inline(always)]
    pub fn adc_drc_signal_delay_time_set(&self) -> ADC_DRC_SIGNAL_DELAY_TIME_SET_R {
        ADC_DRC_SIGNAL_DELAY_TIME_SET_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - DRC delay buffer data output state when The DRC delay function is enabled and the DRC function is disabled. After disabled DRC function and this bit goes to 0, the user should write the DRC delay function bit to 0."]
    #[inline(always)]
    pub fn adc_drc_delay_buf_output_state(&self) -> ADC_DRC_DELAY_BUF_OUTPUT_STATE_R {
        ADC_DRC_DELAY_BUF_OUTPUT_STATE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DRC ET enable\n\nWhen the bit is disabled, Ke and OPE parameter is unused."]
    #[inline(always)]
    pub fn adc_drc_et_en(&mut self) -> ADC_DRC_ET_EN_W<0> {
        ADC_DRC_ET_EN_W::new(self)
    }
    #[doc = "Bit 1 - DRC LT enable\n\nWhen the bit is disabled, Kl and OPL parameter is unused."]
    #[inline(always)]
    pub fn adc_drc_lt_en(&mut self) -> ADC_DRC_LT_EN_W<1> {
        ADC_DRC_LT_EN_W::new(self)
    }
    #[doc = "Bit 2 - Delay function enable\n\nWhen the bit is disabled, the signal delay time is unused."]
    #[inline(always)]
    pub fn adc_drc_delay_func_en(&mut self) -> ADC_DRC_DELAY_FUNC_EN_W<2> {
        ADC_DRC_DELAY_FUNC_EN_W::new(self)
    }
    #[doc = "Bit 3 - Signal function select\n\nWhen the signal function selects the Peak filter, the RMS parameter is unused. (AC_DRC_LRMSHAT, AC_DRC_LRMSLAT, AC_DRC_LRMSHAT, and AC_DRC_LRMSLAT)\n\nWhen the signal function selects the RMS filter, the Peak filter parameter is unused. (AC_DRC_LPFHAT, AC_DRC_LPFLAT, AC_DRC_RPFHAT, AC_DRC_RPFLAT, AC_DRC_LPFHRT, AC_DRC_LPFLRT, AC_DRC_RPFHRT, and AC_DRC_RPFLRT)"]
    #[inline(always)]
    pub fn adc_drc_signal_func_sel(&mut self) -> ADC_DRC_SIGNAL_FUNC_SEL_W<3> {
        ADC_DRC_SIGNAL_FUNC_SEL_W::new(self)
    }
    #[doc = "Bit 4 - Control the DRC to detect noise when ET is enabled"]
    #[inline(always)]
    pub fn adc_drc_detect_noise_en(&mut self) -> ADC_DRC_DETECT_NOISE_EN_W<4> {
        ADC_DRC_DETECT_NOISE_EN_W::new(self)
    }
    #[doc = "Bit 5 - DRC gain min limit enable\n\nWhen this fuction is enabled, it will overwrite the noise detect function."]
    #[inline(always)]
    pub fn adc_drc_gain_min_limit_en(&mut self) -> ADC_DRC_GAIN_MIN_LIMIT_EN_W<5> {
        ADC_DRC_GAIN_MIN_LIMIT_EN_W::new(self)
    }
    #[doc = "Bit 6 - DRC gain max limit enable"]
    #[inline(always)]
    pub fn adc_drc_gain_max_limit_en(&mut self) -> ADC_DRC_GAIN_MAX_LIMIT_EN_W<6> {
        ADC_DRC_GAIN_MAX_LIMIT_EN_W::new(self)
    }
    #[doc = "Bit 7 - The delay buffer use or not when the DRC is disabled and the DRC buffer data output completely."]
    #[inline(always)]
    pub fn adc_drc_delay_buf_en(&mut self) -> ADC_DRC_DELAY_BUF_EN_W<7> {
        ADC_DRC_DELAY_BUF_EN_W::new(self)
    }
    #[doc = "Bits 8:13 - Signal delay time setting\n\nDelay time = 8*(n + 1) fs, n < 6'h30\n\nWhen the delay function is disabled, the signal delay time is unused."]
    #[inline(always)]
    pub fn adc_drc_signal_delay_time_set(&mut self) -> ADC_DRC_SIGNAL_DELAY_TIME_SET_W<8> {
        ADC_DRC_SIGNAL_DELAY_TIME_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC DRC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_adc_drc_ctrl](index.html) module"]
pub struct AC_ADC_DRC_CTRL_SPEC;
impl crate::RegisterSpec for AC_ADC_DRC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_adc_drc_ctrl::R](R) reader structure"]
impl crate::Readable for AC_ADC_DRC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_adc_drc_ctrl::W](W) writer structure"]
impl crate::Writable for AC_ADC_DRC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ac_adc_drc_ctrl to value 0x80"]
impl crate::Resettable for AC_ADC_DRC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
